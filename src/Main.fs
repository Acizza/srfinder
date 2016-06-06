module Main

open Airport
open Argument
open Util
open Util.Type

let processAirports origin mach options airports =
    let (departure, airports) =
        airports
        |> Array.partition (fun a -> a.ICAO = origin)
        |> fun (dep, airports) -> (Array.tryHead dep, airports)

    match departure with
    | Some dep ->
        let sortType =
            options
            |> List.tryPick (function | SortBy x -> Some x | _ -> None)
            |> Option.defaultArg Time

        airports
        |> Route.filter dep options mach
        |> Route.display sortType dep mach
    | None -> printfn "Departure ICAO \"%s\" not found" origin

[<EntryPoint>]
let main args =
    let (|Arguments|_|) list =
        match Argument.parse list with
        | [] -> None
        | xs -> Some xs

    match args |> Array.toList with
    | "find" :: origin :: Double mach :: Arguments args ->
        Airport.loadAll "airports.csv"
        |> Seq.toArray
        |> processAirports origin mach args
    | "random" :: Double mach :: Arguments args ->
        let airports =
            Airport.loadAll "airports.csv"
            |> Seq.toArray

        let origin = Airport.getRandom airports
        processAirports origin.ICAO mach args airports
    | mode :: _ -> printfn "Unknown mode \"%s\". Valid modes are \"find\" and \"random\"" mode
    | _ -> printfn "Usage: <mode> <filters>"

    0