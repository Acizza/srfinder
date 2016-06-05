module Main

open Airport
open Argument
open Util
open Util.Type

let processRoutes origin mach options =
    let (departure, airports) =
        Airport.loadAll "airports.csv"
        |> Seq.toArray
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
    match args |> Array.toList with
    | origin :: Double mach :: xs ->
        let args = Argument.parse xs
        match args with
        | [] -> printfn "No filters specified"
        | _ -> processRoutes origin mach args
    | [] | _ -> printfn "Usage: <departure ICAO> <cruise speed> <filters>"

    0