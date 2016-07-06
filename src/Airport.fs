module Airport

open FSharp.Data
open System
open System.IO
open System.Net
open Util
open Util.Result

type Airports = CsvProvider<Schema = ",Ident (string),Type (string),Name (string),Lat (float),Lon (float),,Continent (string)",
                            CacheRows = false, HasHeaders = false>

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
                    (double x.Lat)
                    (double x.Lon)
        Type      = readType x.Type
        Continent = x.Continent
    }

    (Airports.Load path).Rows
    |> Seq.tail // Skip the row containing the headers
    |> Seq.map createInfo

let isOldDataFile path =
    let info = FileInfo path
    not info.Exists || (DateTime.UtcNow - info.LastWriteTimeUtc).TotalDays > 30.

let tryUpdateDataFile path =
    try
        use wc = new WebClient()
        wc.DownloadFile("http://ourairports.com/data/airports.csv", path)
        Success ()
    with
    | ex -> Failure ex.Message

let findFromICAO icao = Array.tryFind (fun a -> a.ICAO = icao)