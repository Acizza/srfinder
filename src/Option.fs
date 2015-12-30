module Option

open System

type Option =
    | MinTime         of TimeSpan
    | MaxTime         of TimeSpan
    | ArriveBefore    of TimeSpan
    | OriginContinent of string
    | DestContinent   of string
    | OriginAirport   of string
    | DestAirport     of string

/// Parses every parameter in the specified list that starts with '-'
let readAll args =
    let rec readAllRec = function
        | (name:string)::value::xs when name.StartsWith "-" ->
            let opt =
                match name.[1..] with
                | "min"      -> MinTime (TimeSpan.FromHours (float value)) |> Some
                | "max"      -> MaxTime (TimeSpan.FromHours (float value)) |> Some
                | "arrivebf" -> ArriveBefore (TimeSpan.FromHours (float value)) |> Some
                | "oc"       -> OriginContinent value |> Some
                | "dc"       -> DestContinent value |> Some
                | "origin"   -> OriginAirport value |> Some
                | "dest"     -> DestAirport value |> Some
                | _        -> None
            opt::readAllRec xs
        | _ -> []

    readAllRec args
    |> List.choose id
