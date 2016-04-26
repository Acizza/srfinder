module Filter

open System

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

    let rec readAllRec = function
        | (name:string)::value::xs when name.StartsWith "-" ->
            let opt =
                match name.[1..] with
                | "min"      -> MinTime (TimeSpan.FromHours (float value)) |> Some
                | "max"      -> MaxTime (TimeSpan.FromHours (float value)) |> Some
                | "arrivebf" -> ArriveBefore (TimeSpan.FromHours (float value)) |> Some
                | "dc"       -> DepartureContinent value |> Some
                | "ac"       -> ArrivalContinent value |> Some
                | "dest"     -> ArrivalAirport value |> Some
                | "type"     -> readAirportType value |> Option.map AirportType
                | _          -> None
            opt::readAllRec xs
        | _ -> []

    readAllRec args
    |> List.choose id