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
    | DepCont of string
    | ArrCont of string
    | DepType of string
    | ArrType of string
    | Sort    of string
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

let (|Sorter|_|) (v : string) =
    match v.ToLower() with
    | "time"     -> Some Time
    | "name"     -> Some Name
    | "icao"     -> Some ICAO
    | _          -> None

let readArguments args =
    let parser = ArgumentParser.Create<CmdArguments>()
    try
        parser.Parse(args).GetAllResults() |> Some
    with
    | :? ArgumentException ->
        parser.Usage() |> eprintfn "Usage:%s"
        None
    | _ -> None

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
            | Sort    (Sorter s)      -> Filter.SortBy s             |> Some
            | _ -> None
        )

    let airports =
        Airport.loadAll "airports.csv"
        |> Seq.toArray

    let departure =
        List.tryPick (function | Departure d -> Some d | _ -> None) args
        |> Option.defaultArg (
            let rec loop () =
                let arpt = Airport.getRandom airports

                let validate = function
                    | DepartureType t      when arpt.Type      <> t -> false
                    | DepartureContinent c when arpt.Continent <> c -> false
                    | _ -> true

                if List.forall validate filters
                then arpt
                else loop ()

            (loop ()).ICAO
        )

    let mach = List.pick (function | Mach m -> Some m | _ -> None) args
    Route.filterAndDisplay departure mach filters airports

[<EntryPoint>]
let main args =
    args
    |> readArguments
    |> Option.iter processAirports

    0