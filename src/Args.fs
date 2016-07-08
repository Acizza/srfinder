module Args

open Argu
open Route
open System
open Util.Type

type CmdArguments =
    | Departure          of icao:string
    | [<Mandatory>] Mach of double
    | Min                of hours:string
    | Max                of hours:string
    | ArriveBefore       of hour:string
    | Arrival            of icao:string
    | DepCont            of string
    | ArrCont            of string
    | DepType            of Airport.Type
    | ArrType            of Airport.Type
    | Sort               of SortType
    | SortOrder          of SortOrder
    | AutoUpdate         of bool
    | MaxRoutes          of int
    | NumLegs            of int
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
                | NumLegs _      -> "specify the number of legs to find"

//** These serve as a hack to get the type system to automatically infer the type of args when called
let getValue (arg : Quotations.Expr<('Field -> _)>) (args : ParseResults<_>) =
    args.GetResult arg

let tryGetValue (arg : Quotations.Expr<('Field -> _)>) (args : ParseResults<_>) =
    args.TryGetResult arg
//**

let readAll args =
    let parser = ArgumentParser.Create<CmdArguments>()
    try
        parser.Parse(args) |> Some
    with
    | :? ArguParseException ->
        parser.PrintUsage() |> eprintfn "%s"
        None
    | _ -> None

let parseFilters (args : ParseResults<_>) =
    let (|Time|_|) v =
        match TimeSpan.TryParse v with
        | (true, x) -> Some x
        | (false, _) ->
            // Try to parse a decimal time instead
            match v with
            | Double n -> Some (TimeSpan.FromHours n)
            | _        -> None

    args.GetAllResults()
    |> List.choose (function
        | Min          (Time t) -> Filter.MinTime t            |> Some
        | Max          (Time t) -> Filter.MaxTime t            |> Some
        | ArriveBefore (Time t) -> Filter.ArriveBefore t       |> Some
        | Arrival icao          -> Filter.ArrivalAirport icao  |> Some
        | DepCont c             -> Filter.DepartureContinent c |> Some
        | ArrCont c             -> Filter.ArrivalContinent c   |> Some
        | DepType t             -> Filter.DepartureType t      |> Some
        | ArrType t             -> Filter.ArrivalType t        |> Some
        | _ -> None
    )