module Main

open Airport
open Filter
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
    let (|Filters|_|) list =
        match Filter.parse list with
        | [] -> None
        | xs -> Some xs

    match args |> Array.toList with
    | "depart" :: origin :: Double mach :: Filters args ->
        Airport.loadAll "airports.csv"
        |> Seq.toArray
        |> processAirports origin mach args
    | "depart" :: _ -> printfn "depart usage: <origin ICAO> <mach> <filters>"
    | "random" :: Double mach :: Filters args ->
        let airports =
            Airport.loadAll "airports.csv"
            |> Seq.toArray

        let origin =
            let rec loop () =
                let arpt = Airport.getRandom airports

                let validate = function
                    | DepartureType t      when arpt.Type      <> t -> false
                    | DepartureContinent c when arpt.Continent <> c -> false
                    | _ -> true

                if List.forall validate args
                then arpt
                else loop ()

            loop ()

        processAirports origin.ICAO mach args airports
    | "random" :: _ -> printfn "random usage: <mach> <filters>"
    | mode     :: _ -> printfn "Unknown mode \"%s\". Valid modes are \"depart\" and \"random\"" mode
    | _             -> printfn "Usage: <mode> <mode parameters> <filters>"

    0