module Route

open Airport
open System
open Util
open Util.Convert

type SortType =
    | Time
    | Name
    | ICAO

type SortOrder =
    | Ascending
    | Descending

type Filter =
    | MinTime            of TimeSpan
    | MaxTime            of TimeSpan
    | ArriveBefore       of TimeSpan
    | DepartureContinent of string
    | ArrivalContinent   of string
    | ArrivalAirport     of string
    | DepartureType      of Airport.Type
    | ArrivalType        of Airport.Type

type RouteInfo = {
    Mach      : float
    Filters   : Filter list
    SortType  : SortType
    SortOrder : SortOrder
}

let getSortType (v : string) =
    match v.ToLower() with
    | "time"     -> Some Time
    | "name"     -> Some Name
    | "icao"     -> Some ICAO
    | _          -> None

let getSortOrder (v : string) =
    match v.ToLower() with
    | "ascending"  -> Some Ascending
    | "descending" -> Some Descending
    | _            -> None

let filter info departure airports =
    let (|TimeBetween|) = Airport.timeBetween info.Mach departure

    let filter = function
        | TimeBetween t, MinTime m       -> t >= m
        | TimeBetween t, MaxTime m       -> t <= m
        | TimeBetween t, ArriveBefore lt -> DateTime.Now.TimeOfDay + t <= lt
        | arrival, ArrivalContinent c    -> arrival.Continent = c
        | arrival, ArrivalAirport a      -> arrival.ICAO = a
        | arrival, ArrivalType t         -> arrival.Type = t
        | _, DepartureContinent _
        | _, DepartureType _             -> true

    airports
    |> Array.filter (fun arrival ->
        arrival.ICAO <> departure.ICAO &&
        List.forall (fun option -> filter (arrival, option)) info.Filters
    )

let display info departure airports =
    let timeToArpt = Airport.timeBetween info.Mach departure

    let sorter x y =
        let (x, y) =
            if info.SortOrder = Descending
            then (y, x)
            else (x, y)

        match info.SortType with
        | Time -> compare (timeToArpt y) (timeToArpt x)
        | Name -> compare x.Name y.Name
        | ICAO -> compare x.ICAO y.ICAO

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

let filterAndDisplay info origin airports =
    let (departure, airports) =
        airports
        |> Array.partition (fun a -> a.ICAO = origin)
        |> fun (dep, airports) -> (Array.tryHead dep, airports)

    match departure with
    | Some dep ->
        airports
        |> filter info dep
        |> display info dep
        Success ()
    | None -> Failure (sprintf "Departure ICAO \"%s\" not found" origin)

let rec randomDeparture filters airports =
    let arpt = Airport.getRandom airports

    let validate = function
        | DepartureType t      when arpt.Type      <> t -> false
        | DepartureContinent c when arpt.Continent <> c -> false
        | _ -> true

    if List.forall validate filters
    then arpt
    else randomDeparture filters airports

let rec displayRandom info airports = function
    | 0 -> Failure "No routes found"
    | maxTries ->
        let departure = randomDeparture info.Filters airports
        let routes    = filter info departure airports

        match routes.Length with
        | 0 -> displayRandom info airports (maxTries - 1)
        | _ ->
            display info departure routes
            Success ()