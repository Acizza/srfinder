module Route

open Airport
open System
open Util
open Util.Convert
open Util.Result

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

let filterArrivals info departure airports =
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

let filterDepartures filters airports =
    let validate arpt = function
        | DepartureType t      when arpt.Type      <> t -> false
        | DepartureContinent c when arpt.Continent <> c -> false
        | _ -> true

    airports
    |> Array.filter (fun arpt -> List.forall (validate arpt) filters)

let randomDeparture filters airports =
    match filterDepartures filters airports with
    | [||]  -> None
    | arpts -> Some (Array.random arpts)

module private Util =
    /// Formats a TimeSpan into an hour:minute formatted string
    let formatTime (t : TimeSpan) = t.ToString "h\:mm"

module Leg =
    let display maxRoutes info departure airports =
        let timeToArpt = Airport.timeBetween info.Mach departure

        let sorter x y =
            let (x, y) =
                if info.SortOrder = Descending
                then (y, x)
                else (x, y)

            match info.SortType with
            | Time -> compare (timeToArpt x) (timeToArpt y)
            | Name -> compare x.Name y.Name
            | ICAO -> compare x.ICAO y.ICAO

        let takeMax =
            match maxRoutes with
            | Some m -> Array.upTo m
            | None   -> id

        let processedAirports =
            airports
            |> Array.sortWith sorter
            |> takeMax

        let numRoutesStr =
            match maxRoutes with
            | Some m -> sprintf "%d of %d"
                            <| Array.length processedAirports
                            <| Array.length airports
            | None   -> sprintf "%d" (Array.length airports)

        printfn "Displaying %s routes from %s (%s):\n"
            numRoutesStr
            departure.ICAO
            departure.Name

        processedAirports
        |> Array.iter (fun arr ->
            printfn "*****\nName: %s\nICAO: %s\nTime: %s\nDist: %.0fnm\n*****\n"
                arr.Name
                arr.ICAO
                (timeToArpt arr |> Util.formatTime)
                (Airport.distance departure arr |> Meter.toNauticalMiles)
        )

    let filterAndDisplay maxRoutes info dep =
        filterArrivals info dep
        >> display maxRoutes info dep

module Segment =
    let find numLegs info departure airports =
        let rec findRec lastDep = function
            | 0 -> []
            | numLegs ->
                match filterArrivals info lastDep airports with
                | [||]  -> []
                | arpts -> lastDep :: findRec (Array.random arpts) (numLegs - 1)
 
        match findRec departure (numLegs + 1) with
        | []  -> Failure "No flight segment found"
        | [_] -> Failure "Flight segment too short"
        | seg ->
            // Group the legs into [(x, y); (y, z)] form for easy iteration
            List.zip
                seg.[..seg.Length-2]
                (List.tail seg)
            |> Success

    let display legs info airports =
        let timeToArpt = Airport.timeBetween info.Mach

        List.length legs
        |> printfn "Displaying %d legs:\n"

        legs
        |> List.iteri (fun i (dep, arr) ->
            printfn "Leg %d:\n  * Route: %s -> %s\n  * Time: %s\n  * Distance: %.0fnm\n"
                (i + 1)
                dep.ICAO
                arr.ICAO
                (timeToArpt dep arr |> Util.formatTime)
                (Airport.distance dep arr |> Meter.toNauticalMiles)
        )

        legs
        |> List.map ((<||) timeToArpt)
        |> List.reduce (+)
        |> Util.formatTime
        |> printfn "Total time: %s"

        legs
        |> List.map ((<||) Airport.distance >> Meter.toNauticalMiles)
        |> List.sum
        |> printfn "Total distance: %.0fnm"

    let findAndDisplay numLegs info dep airports =
        find numLegs info dep airports
        |> Result.map (fun legs -> display legs info airports)