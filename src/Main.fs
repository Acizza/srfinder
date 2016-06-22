module Main

open Airport
open Argu
open Route
open System
open Util
open Util.Type

type CmdArguments =
    | [<PrintLabels>] Departure    of icao:string
    | [<Mandatory>]   Mach         of double
    | [<PrintLabels>] Min          of hours:string
    | [<PrintLabels>] Max          of hours:string
    | [<PrintLabels>] ArriveBefore of hour:string
    | [<PrintLabels>] Arrival      of icao:string
    | DepCont    of string
    | ArrCont    of string
    | DepType    of string
    | ArrType    of string
    | Sort       of string
    | SortOrder  of string
    | AutoUpdate of bool
    with
        interface IArgParserTemplate with
            member s.Usage =
                match s with
                | Departure _    -> "specify a departure airport"
                | Mach _         -> "set the cruising speed"
                | Min _          -> "specify the minimum time for routes"
                | Max _          -> "specify the maximum time for routes"
                | ArriveBefore _ -> "set the local time that routes must arrive by"
                | Arrival _      -> "set the arrival airport"
                | DepCont _      -> "set the continent for the departure airport"
                | ArrCont _      -> "set the continent for the arrival airport"
                | DepType _      -> "set the type of airport for the departure"
                | ArrType _      -> "set the type of airport for the arrival"
                | Sort _         -> "specify how routes should be displayed"
                | SortOrder _    -> "specifiy if you want routes displayed in ascending or descending order"
                | AutoUpdate _   -> "set whether or not airport data will be automatically updated"

let (|Time|_|) v =
    match TimeSpan.TryParse v with
    | (true, x) -> Some x
    | (false, _) ->
        // Try to parse a decimal time instead
        match v with
        | Double n -> Some (TimeSpan.FromHours n)
        | _        -> None

let (|AirportType|_|) (v : string) =
    match v.ToLower() with
    | "closed"   -> Some Airport.Closed
    | "heliport" -> Some Airport.Heliport
    | "small"    -> Some Airport.Small
    | "medium"   -> Some Airport.Medium
    | "large"    -> Some Airport.Large
    | _ -> None

let readArguments args =
    let parser = ArgumentParser.Create<CmdArguments>()
    try
        parser.Parse(args) |> Some
    with
    | :? ArgumentException ->
        parser.Usage() |> eprintfn "Usage:%s"
        None
    | _ -> None

let airportDataPath =
    sprintf "%s%s"
        AppDomain.CurrentDomain.BaseDirectory
        "airports.csv"

let processAirports args =
    let filters =
        args
        |> List.choose (function
            | Min          (Time t)   -> Filter.MinTime t            |> Some
            | Max          (Time t)   -> Filter.MaxTime t            |> Some
            | ArriveBefore (Time t)   -> Filter.ArriveBefore t       |> Some
            | Arrival icao            -> Filter.ArrivalAirport icao  |> Some
            | DepCont c               -> Filter.DepartureContinent c |> Some
            | ArrCont c               -> Filter.ArrivalContinent c   |> Some
            | DepType (AirportType t) -> Filter.DepartureType t      |> Some
            | ArrType (AirportType t) -> Filter.ArrivalType t        |> Some
            | _ -> None
        )

    let airports =
        Airport.loadAll airportDataPath
        |> Seq.toArray

    let routeInfo = {
        Mach     = List.pick (function | Mach m -> Some m | _ -> None) args
        Filters  = filters
        SortType =
            args
            |> List.tryPick (function | Sort s -> Some s | _ -> None)
            |> Option.bind Route.getSortType
            |> Option.defaultArg Time
        SortOrder =
            args
            |> List.tryPick (function | SortOrder o -> Some o | _ -> None)
            |> Option.bind Route.getSortOrder
            |> Option.defaultArg Ascending
    }

    let result =
        let departure = List.tryPick (function | Departure d -> Some d | _ -> None) args
        match departure with
        | Some dep -> Route.filterAndDisplay routeInfo dep airports
        | None     -> Route.displayRandom routeInfo airports 10

    match result with
    | Success _   -> ()
    | Failure msg -> eprintfn "Failure during route processing: %s" msg

let checkAndupdateAirportData () =
    match Airport.isOldDataFile airportDataPath with
    | true ->
        printfn "Airport data out of date. Updating.."
        match Airport.tryUpdateDataFile airportDataPath with
        | Success _   -> ()
        | Failure msg -> eprintfn "Error updating airport data: %s" msg
    | false -> ()

[<EntryPoint>]
let main args =
    match readArguments args with
    | Some results ->
        let autoUpdateEnabled = results.GetResult(<@ AutoUpdate @>, defaultValue = true)
        if autoUpdateEnabled then
            checkAndupdateAirportData ()

        results.GetAllResults()
        |> processAirports
    | None -> ()

    0