module Main

open System
open Filter
open Airport
open Nessos.Streams
open Util.Convert

let filterAirports options mach airports =
    let (|TimeBetween|) (dInfo, aInfo) = Airport.timeBetween mach dInfo aInfo
    let (|Arrival|) = snd

    let filter = function
        | TimeBetween t, MinTime m          -> t >= m
        | TimeBetween t, MaxTime m          -> t <= m
        | TimeBetween t, ArriveBefore lt    -> DateTime.Now.TimeOfDay + t <= lt
        | Arrival aInfo, DepartureContinent c
        | Arrival aInfo, ArrivalContinent c -> aInfo.Continent = c
        | Arrival aInfo, ArrivalAirport a   -> aInfo.ICAO = a
        | Arrival aInfo, AirportType t      -> aInfo.Type = t

    airports
    |> ParStream.filter (fun (dInfo, aInfo) ->
        List.forall (fun o ->
            filter ((dInfo, aInfo), o)
        ) options
    )

/// Convert a decimal containing hours to it's hour:minute representation
let decToTime hours =
    let minutes = (hours % 1.) * 60.
    sprintf "%d:%02d"
        (floor hours |> int)
        (floor minutes |> int)

let processRoutes origin mach options =
    let filterAll =
        ParStream.ofSeq
        >> ParStream.unordered
        >> filterAirports options mach

    let display =
        ParStream.sortByDescending ((<||) (Airport.timeBetween mach))
        >> ParStream.toArray
        >> Array.iter (fun (d, a) ->
            printfn "*****\nName: %s\nICAO: %s\nTime: %s\nDist: %.0fnm\n*****\n"
                a.Name
                a.ICAO
                (decToTime (Airport.timeBetween mach d a).TotalHours)
                (Airport.distance d a |> Meter.toNauticalMiles)
        )

    let airports  = Airport.loadAll "airports.csv"
    let departure = Seq.tryFind (fun a -> a.ICAO = origin) airports

    match departure with
    | Some dep ->
        airports
        |> Seq.filter (fun a -> a.ICAO <> dep.ICAO)
        |> Seq.map (fun a -> (dep, a))
        |> filterAll
        |> display
    | None -> ()

[<EntryPoint>]
let main args =
    let tryParse f x =
        match f x with
        | (true, v) -> Some v
        | (false, _) -> None

    let (|Double|_|) = tryParse Double.TryParse

    match args |> Array.toList with
    | origin::Double mach::xs ->
        let filters = Filter.readAll xs
        match filters with
        | [] -> Console.WriteLine "No filters specified"
        | _ -> processRoutes origin mach filters
    | [] | _ -> Console.WriteLine "Usage: <departure ICAO> <cruise speed> <filters>"

    0