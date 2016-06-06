module Route

open Airport
open Filter
open System
open Util
open Util.Convert

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