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
let findFromICAO icao = Array.tryFind (fun a -> a.ICAO = icao)

let timeBetween mach from to' =
    let metersPerSecond = 343.
    let distance = distance from to'
    TimeSpan.FromSeconds(distance / (mach * metersPerSecond))

let loadAll (path : string) =
    let readType = function
        | "closed"         -> Closed
        | "heliport"       -> Heliport
        | "small_airport"  -> Small
        | "medium_airport" -> Medium
        | "large_airport"  -> Large
        | _                -> Closed

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

module DataFile =
    let name = "airports.csv"
    let path = Util.localPath name

    let isOld path =
        let info = FileInfo path
        not info.Exists || (DateTime.UtcNow - info.LastWriteTimeUtc).TotalDays > 30.

    let update () =
        try
            use wc = new WebClient()
            wc.DownloadFile(
                sprintf "http://ourairports.com/data/%s" name,
                path)
            Success ()
        with
        | ex -> Failure ex.Message

    let verifyAndUpdate () =
        match isOld path with
        | true ->
            printfn "Airport data out of date. Updating.."
            update ()
        | false -> Success ()