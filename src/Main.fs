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
    | MaxRoutes  of int
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
                | MaxRoutes _    -> "set the maximum number of routes to display"

//** These serve as a hack to get the type system to automatically infer the type of args when called
let getArgValue (arg : Quotations.Expr<('Field -> _)>) (args : ParseResults<_>) =
    args.GetResult arg

let tryGetArgValue (arg : Quotations.Expr<('Field -> _)>) (args : ParseResults<_>) =
    args.TryGetResult arg
//**

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

let processAirports (args : ParseResults<_>) =
    let filters =
        args.GetAllResults()
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
        Mach     = getArgValue <@ Mach @> args
        Filters  = filters
        SortType =
            args
            |> tryGetArgValue <@ Sort @>
            |> Option.bind Route.getSortType
            |> Option.defaultArg Time
        SortOrder =
            args
            |> tryGetArgValue <@ SortOrder @>
            |> Option.bind Route.getSortOrder
            |> Option.defaultArg Descending
    }

    let result =
        let departure = tryGetArgValue <@ Departure @> args
        let maxRoutes = tryGetArgValue <@ MaxRoutes @> args

        match departure with
        | Some dep -> Route.filterAndDisplay maxRoutes routeInfo dep airports
        | None     -> Route.displayRandom maxRoutes routeInfo airports 10

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
        let autoUpdateEnabled =
            tryGetArgValue (<@ AutoUpdate @>) results
            |> Option.defaultArg true

        if autoUpdateEnabled then
            checkAndupdateAirportData ()

        processAirports results
    | None -> ()

    0