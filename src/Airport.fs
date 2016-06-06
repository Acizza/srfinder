module Airport

open FSharp.Data
open System
open Util

type Airports = CsvProvider<"airports.csv", CacheRows = false>

type Type =
    | Closed
    | Heliport
    | Small
    | Medium
    | Large

type Airport = {
    Name      : string
    ICAO      : string
    Coord     : Coord.T
    Type      : Type
    Continent : string
}

let distance from to' = Coord.distance from.Coord to'.Coord

let timeBetween mach from to' =
    let metersPerSecond = 343.
    let distance = distance from to'
    TimeSpan.FromSeconds(distance / (mach * metersPerSecond))

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

let getRandom =
    let rng = new System.Random()
    fun (airports : Airport array) ->
        airports.[rng.Next(0, airports.Length)]