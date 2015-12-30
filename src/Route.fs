module Route

open System
open FSharp.Data

type FSCloud = HtmlProvider<"http://fscloud-infotool.de/index.php?page=vasystem&subpage=vadetails&id=10277">

type Route = {
    origin : string
    dest   : string
    route  : string
    time   : TimeSpan
}

/// Returns a URL to a specified FSCloud VA from its ID
let getVAAddress id =
    "http://fscloud-infotool.de/index.php?page=vasystem&subpage=vadetails&id="
    + id

/// Parses every flight route from the specified FSCloud Virtual Airline ID
let parseAll id =
    let addr = getVAAddress id
    let fs = FSCloud.Load(addr).Tables.Table6
    Seq.map (fun (x:FSCloud.Table6.Row) ->
        {
            origin = x.Origin
            dest = x.Destination
            route = x.``Recommended Flight Plan``
            time = TimeSpan.FromMinutes (x.``Flight Time``.Split(' ').[0] |> float)
        }
        ) fs.Rows
