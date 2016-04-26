module Airport

open System
open FSharp.Data

module Coord =
    type T = {
        Lat  : double
        Lon : double
    }

    let create lat lon = {
        Lat = lat
        Lon = lon
    }

    /// Returns the distance between two coordinates in meters
    let distance c1 c2 =
        // Haversine formula
        let toRads deg = deg * Math.PI / 180.
        let r = 6371000. // Earth's radius in meters

        let lat1 = toRads c1.Lat
        let lat2 = toRads c2.Lat
        let lat  = toRads (c2.Lat - c1.Lat)
        let lon  = toRads (c2.Lon - c1.Lon)

        let a = sin (lat / 2.) * sin (lat / 2.) +
                cos lat1      * cos lat2      *
                sin (lon / 2.) * sin (lon / 2.)

        let c = 2. * atan2 (sqrt a) (sqrt (1. - a))
        r * c

type Airports = CsvProvider<"airports.csv">

type Type =
    | Closed
    | Heliport
    | Small
    | Medium
    | Large

type Info = {
    Name      : string
    ICAO      : string
    Coord     : Coord.T
    Type      : Type
    Continent : string
}

let distance from to' = Coord.distance from.Coord to'.Coord

let timeBetween mach from to' =
    let metersPerHour = 1.235e6
    let distance = distance from to'
    TimeSpan.FromHours(distance / (mach * metersPerHour))

let readType = function
    | "closed"         -> Closed
    | "heliport"       -> Heliport
    | "small_airport"  -> Small
    | "medium_airport" -> Medium
    | "large_airport"  -> Large
    | _                -> Closed

let loadAll (path : string) =
    let createInfo (x : Airports.Row) = {
        Name  = x.Name
        ICAO  = x.Ident
        Coord = Coord.create
                    (double x.Latitude_deg)
                    (double x.Longitude_deg)
        Type      = readType x.Type
        Continent = x.Continent
    }

    (Airports.Load path).Rows
    |> Seq.map createInfo