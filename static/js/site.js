// TODO: cleanup
var setDepartureICAO = null;
var routeData = [];

require([
    "esri/Map",
    "esri/views/MapView",
    "esri/geometry/Point",
    "esri/geometry/Polyline",
    "esri/geometry/geometryEngine",
    "esri/symbols/SimpleMarkerSymbol",
    "esri/symbols/SimpleLineSymbol",
    "esri/Graphic",
    "dojo/domReady!"
], function(
    Map,
    MapView,
    Point,
    Polyline,
    geometryEngine,
    SimpleMarkerSymbol,
    SimpleLineSymbol,
    Graphic,
) {
    function drawRoute(startPoint, endPoint, view) {
        var marker = new SimpleMarkerSymbol({
            style: "diamond",
            size:  "10px"
        });

        var linePath = new Polyline({
            paths: [[startPoint.x, startPoint.y], [endPoint.x, endPoint.y]]
        });

        var lineSymbol = new SimpleLineSymbol({
            width: 2
        });

        var geodesicLine = geometryEngine.geodesicDensify(linePath, 100);

        view.graphics.add(new Graphic(startPoint, marker));
        view.graphics.add(new Graphic(endPoint, marker));
        view.graphics.add(new Graphic(geodesicLine, lineSymbol));
    }

    var map = new Map({
        basemap: "gray-vector"
    });

    var view = new MapView({
        container: "mapDiv",
        map: map
    });

    // Highlight route and draw it on the map
    $("#route-table").on("mouseenter", ".route-data", function() {
        $('.highlight').removeClass('highlight');
        $(this).addClass('highlight');

        view.graphics.removeAll();

        var departure = $(this).children(".departure");
        var depPoint  = new Point(departure.attr("data-lon"), departure.attr("data-lat"));

        var arrival  = $(this).children(".arrival");
        var arrPoint = new Point(arrival.attr("data-lon"), arrival.attr("data-lat"));

        drawRoute(depPoint, arrPoint, view);
    });

    // Populate airport info
    $("#route-table").on("click", ".route-data", function() {
        resetScrollbar($("#route-viewer #scrollable"));

        var route = routeData[$(this).index() - 1];
        
        if(setDepartureICAO != route.departure.icao) {
            populateAirportInfo("#route-viewer #departure", route.departure);
            setDepartureICAO = route.departure.icao;
        }

        populateAirportInfo("#route-viewer #arrival", route.arrival);
    });

    function populateAirportInfo(baseName, airport) {
        $(baseName + " #name").text(airport.name);

        if(airport.frequencies) {
            var freqBase = baseName + " #freq-table";
            var setFreq = function(name, val) {
                $(freqBase + " #" + name).text(val || "n/a");
            };

            setFreq("atis", airport.frequencies.atis);
            setFreq("ground", airport.frequencies.ground);
            setFreq("tower", airport.frequencies.tower);
            setFreq("dep", airport.frequencies.departure);
            setFreq("app", airport.frequencies.approach);
        }

        if(airport.runways) {
            var runwayTable = $(baseName + " #runway-table")[0];
            clearTable(runwayTable);

            for(i = 0; i < airport.runways.length; ++i) {
                var runway = airport.runways[i];
                var row = runwayTable.insertRow();
                
                var name = row.insertCell(0);
                name.innerHTML = runway.ident.north + " / " + runway.ident.south;
                name.className = "data-value";

                var length = row.insertCell(1);
                length.innerHTML = runway.length ? runway.length + " ft" : "n/a";
                length.className = "data-value";

                var width = row.insertCell(2);
                width.innerHTML = runway.width ? runway.width + " ft" : "n/a";
                width.className = "data-value";

                var open = row.insertCell(3);
                open.innerHTML = runway.closed ? "No" : "Yes";
                open.className = "data-value";
            }
        }
    }
});

$(document).ready(function() {
    var routeSelectorScrollbar = $("#route-selector #scrollable");

    $("#filters #scrollable").perfectScrollbar();
    routeSelectorScrollbar.perfectScrollbar();
    $("#route-viewer #scrollable").perfectScrollbar();

    $("#filters select[name$=_country]").val(null);

    var routeTable = $("#route-selector #route-table");
    var machInput  = $("#filters input[name=mach]");

    $("#filters").submit(function(e) {
        var mach = machInput.val();

        $.ajax({
            type: 'post',
            url:  '/filter',
            data: $(this).serialize(),
            success: function(routes) {
                clearTable(routeTable[0]);
                resetScrollbar(routeSelectorScrollbar);

                routeData = [];

                for(i = 0; i < routes.length; ++i) {
                    insertRoute(routes[i], mach);
                    routeData.push(routes[i]);
                }
            },
            error: function(req, errText, err) {
                console.log("error filtering routes: " + req.responseText);
            }
        });

        e.preventDefault();
    });

    var findButton = $("#filters input[type=submit]");

    // Don't allow submit when the speed isn't provided
    machInput.on("input", function() {
        findButton.prop("disabled", $(this).val() == "");
    });

    function insertRoute(route, mach) {
        var row = routeTable[0].insertRow();
        row.className = "route-data";
        
        var depRow = row.insertCell(0);
        depRow.innerHTML = route.departure.icao;
        depRow.className = "departure";
        addRouteLocation(route.departure.pos, depRow);

        var arrRow = row.insertCell(1);
        arrRow.innerHTML = route.arrival.icao;
        arrRow.className = "arrival";
        addRouteLocation(route.arrival.pos, arrRow);

        var distRow = row.insertCell(2);
        distRow.innerHTML = Math.round(route.distance) + " nm";
        
        var timeRow = row.insertCell(3);
        timeRow.innerHTML = formatRouteTime(route.time, mach);
    }

    function addRouteLocation(pos, row) {
        row.setAttribute("data-lat", pos.lat);
        row.setAttribute("data-lon", pos.lon);
    }

    function formatRouteTime(time, mach) {
        var hours   = Math.floor(time);
        var minutes = Math.floor((time - hours) * 60);

        return hours + ":" + (minutes < 10 ? "0" + minutes : minutes);
    }
});

function clearTable(table) {
    for(i = table.rows.length - 1; i > 0; --i) {
        table.deleteRow(i);
    }
}

function resetScrollbar(elem) {
    elem.scrollTop(0);
    elem.perfectScrollbar('update');
}