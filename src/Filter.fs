module Filter

open System
open Util.Type

type Filter =
    | MinTime            of TimeSpan
    | MaxTime            of TimeSpan
    | ArriveBefore       of TimeSpan
    | DepartureContinent of string
    | ArrivalContinent   of string
    | ArrivalAirport     of string
    | AirportType        of Airport.Type

/// Parses every parameter in the specified list that starts with '-'
let readAll args =
    let readAirportType = function
        | "closed"   -> Some Airport.Closed
        | "heliport" -> Some Airport.Heliport
        | "small"    -> Some Airport.Small
        | "medium"   -> Some Airport.Medium
        | "large"    -> Some Airport.Large
        | _ -> None

    let (|Time|_|) (v : string) =
        try
            Some (TimeSpan.Parse v)
        with
        | _ ->
            match v with
            | Double n -> Some (TimeSpan.FromHours n)
            | _        -> None

    let rec parse = function
        | (name : string) :: rest when name.StartsWith "-" ->
            match name.[1..] :: rest with
            | "min"      :: Time time :: xs -> Some (MinTime time)             :: parse xs
            | "max"      :: Time time :: xs -> Some (MaxTime time)             :: parse xs
            | "arrivebf" :: Time time :: xs -> Some (ArriveBefore time)        :: parse xs
            | "dc"       :: value     :: xs -> Some (DepartureContinent value) :: parse xs
            | "ac"       :: value     :: xs -> Some(ArrivalContinent value)    :: parse xs
            | "dest"     :: value     :: xs -> Some (ArrivalAirport value)     :: parse xs
            | "type"     :: value     :: xs -> (readAirportType value |> Option.map AirportType) :: parse xs
            | _ -> []
        | _ -> []

    parse args
    |> List.choose id