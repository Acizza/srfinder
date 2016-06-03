module Route

open Airport
open Argument
open System
open Nessos.Streams
open Util
open Util.Convert

let filter departure options mach airports =
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
        | _,             SortBy _           -> true

    airports
    |> ParStream.ofSeq
    |> ParStream.filter (fun arpt ->
        arpt.ICAO <> departure.ICAO &&
        List.forall (fun o -> filter ((departure, arpt), o)) options
    )

let display sortType departure mach airports =
    let timeToArpt = Airport.timeBetween mach departure

    let sorter x y =
        match sortType with
        | Time -> compare (timeToArpt y) (timeToArpt x)
        | Name -> compare y.Name x.Name
        | ICAO -> compare y.ICAO x.ICAO

    airports
    |> Array.sortWith sorter
    |> Array.iter (fun arr ->
        printfn "*****\nName: %s\nICAO: %s\nTime: %s\nDist: %.0fnm\n*****\n"
            arr.Name
            arr.ICAO
            (formatTime (timeToArpt arr).TotalHours)
            (Airport.distance departure arr |> Meter.toNauticalMiles)
    )