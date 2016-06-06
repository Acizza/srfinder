module Filter

open System
open Util.Type

type SortType =
    | Time
    | Name
    | ICAO

type Filter =
    | MinTime            of TimeSpan
    | MaxTime            of TimeSpan
    | ArriveBefore       of TimeSpan
    | DepartureContinent of string
    | ArrivalContinent   of string
    | ArrivalAirport     of string
    | DepartureType      of Airport.Type
    | ArrivalType        of Airport.Type
    | SortBy             of SortType

let parseAirportType (v : string) =
    match v.ToLower() with
    | "closed"   -> Some Airport.Closed
    | "heliport" -> Some Airport.Heliport
    | "small"    -> Some Airport.Small
    | "medium"   -> Some Airport.Medium
    | "large"    -> Some Airport.Large
    | _ -> None

/// Parses every argument in the specified list that starts with '-'
let parse args =
    let (|Time|_|) v =
        try
            Some (TimeSpan.Parse v)
        with
        | _ ->
            // Try to parse a decimal time instead
            match v with
            | Double n -> Some (TimeSpan.FromHours n)
            | _        -> None

    let rec parse = function
        | (name : string) :: rest when name.StartsWith "-" ->
            match name.[1..].ToLower() :: rest with
            | "min"      :: Time time :: xs -> Some (MinTime time)             :: parse xs
            | "max"      :: Time time :: xs -> Some (MaxTime time)             :: parse xs
            | "arrivebf" :: Time time :: xs -> Some (ArriveBefore time)        :: parse xs
            | "depcont"  :: value     :: xs -> Some (DepartureContinent value) :: parse xs
            | "arrcont"  :: value     :: xs -> Some(ArrivalContinent value)    :: parse xs
            | "dest"     :: value     :: xs -> Some (ArrivalAirport value)     :: parse xs
            | "deptype"  :: value     :: xs -> (parseAirportType value |> Option.map DepartureType) :: parse xs
            | "arrtype"  :: value     :: xs -> (parseAirportType value |> Option.map ArrivalType) :: parse xs
            | "sort"     :: value     :: xs ->
                let sortType =
                    match value.ToLower() with
                    | "time"     -> Some Time
                    | "name"     -> Some Name
                    | "icao"     -> Some ICAO
                    | _          -> None

                (sortType |> Option.map SortBy) :: parse xs
            | _ -> []
        | _ -> []

    parse args
    |> List.choose id