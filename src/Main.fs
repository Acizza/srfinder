module Main

open FSharp.Collections.ParallelSeq
open System
open Filter

/// Convert a decimal containing hours to it's hour:minute representation
let decToTime hours =
    let minutes = (hours % 1.) * 60.
    sprintf "%d:%02d"
        (floor hours |> int)
        (floor minutes |> int)

let processRoutes vaID options =
    let airports = Airport.load "airports.csv"
    (*
    let routes =
        let filter = function
            | {Route.Time = t},   MinTime m         -> t >= m
            | {Route.Time = t},   MaxTime m         -> t <= m
            | {Route.Time = t},   ArriveBefore lt   -> DateTime.Now.TimeOfDay + t <= lt
            | {Route.Origin = o}, OriginContinent c -> Airport.exists o c airports
            | {Route.Dest = d},   DestContinent c   -> Airport.exists d c airports
            | {Route.Origin = o}, OriginAirport a   -> o = a
            | {Route.Dest = d},   DestAirport a     -> d = a

        Route.parseAll vaID
        |> PSeq.filter (fun r -> Seq.forall (fun o -> filter (r, o)) options)
        |> PSeq.sortBy (fun r -> r.Time.TotalMinutes)

    for r in routes do
        printfn "Origin: %s\nDest: %s\nTime: %s\nRoute: %s\n"
            r.Origin
            r.Dest
            (decToTime <| r.Time.TotalHours)
            r.Route
    *)
    ()

[<EntryPoint>]
let main args =
    match args |> Array.toList with
    | [] -> Console.WriteLine "Usage: <VA id> <filters>"
    | vaID::xs ->
        let filters = Filter.readAll xs
        match filters with
        | [] -> Console.WriteLine "No filters specified"
        | _ -> processRoutes vaID filters
    0