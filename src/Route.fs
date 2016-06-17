module Route

open Airport
open System
open Util
open Util.Convert

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

let getSortType filters =
    filters
    |> List.tryPick (function | SortBy x -> Some x | _ -> None)
    |> Option.defaultArg Time

let filter departure options mach airports =
    let (|TimeBetween|) = Airport.timeBetween mach departure

    let filter = function
        | TimeBetween t, MinTime m       -> t >= m
        | TimeBetween t, MaxTime m       -> t <= m
        | TimeBetween t, ArriveBefore lt -> DateTime.Now.TimeOfDay + t <= lt
        | arrival, ArrivalContinent c    -> arrival.Continent = c
        | arrival, ArrivalAirport a      -> arrival.ICAO = a
        | arrival, ArrivalType t         -> arrival.Type = t
        | _, SortBy _
        | _, DepartureContinent _
        | _, DepartureType _             -> true

    airports
    |> Array.filter (fun arrival ->
        arrival.ICAO <> departure.ICAO &&
        List.forall (fun option -> filter (arrival, option)) options
    )

let display sortType departure mach airports =
    let timeToArpt = Airport.timeBetween mach departure

    let sorter x y =
        match sortType with
        | Time -> compare (timeToArpt y) (timeToArpt x)
        | Name -> compare y.Name x.Name
        | ICAO -> compare y.ICAO x.ICAO

    printfn "Displaying %d routes from %s (%s):\n"
        (Array.length airports)
        departure.ICAO
        departure.Name

    airports
    |> Array.sortWith sorter
    |> Array.iter (fun arr ->
        printfn "*****\nName: %s\nICAO: %s\nTime: %s\nDist: %.0fnm\n*****\n"
            arr.Name
            arr.ICAO
            ((timeToArpt arr).ToString "h\:mm")
            (Airport.distance departure arr |> Meter.toNauticalMiles)
    )

let filterAndDisplay origin mach filters airports =
    let (departure, airports) =
        airports
        |> Array.partition (fun a -> a.ICAO = origin)
        |> fun (dep, airports) -> (Array.tryHead dep, airports)

    match departure with
    | Some dep ->
        let sortType = getSortType filters
        airports
        |> filter dep filters mach
        |> display sortType dep mach
    | None -> printfn "Departure ICAO \"%s\" not found" origin

let rec randomDeparture filters airports =
    let arpt = Airport.getRandom airports

    let validate = function
        | DepartureType t      when arpt.Type      <> t -> false
        | DepartureContinent c when arpt.Continent <> c -> false
        | _ -> true

    if List.forall validate filters
    then arpt
    else randomDeparture filters airports

let rec displayRandom mach filters airports = function
    | 0 -> printfn "No routes found"
    | maxTries ->
        let departure = randomDeparture filters airports
        let routes    = filter departure filters mach airports

        match routes.Length with
        | 0 -> displayRandom mach filters airports (maxTries - 1)
        | _ -> display (getSortType filters) departure mach routes