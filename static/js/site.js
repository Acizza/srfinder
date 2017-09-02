require([
    "esri/Map",
    "esri/views/MapView",
    "esri/widgets/BasemapToggle",
    "esri/geometry/Point",
    "esri/geometry/Polyline",
    "esri/geometry/geometryEngine",
    "esri/layers/GraphicsLayer",
    "esri/symbols/SimpleMarkerSymbol",
    "esri/symbols/SimpleLineSymbol",
    "esri/symbols/TextSymbol",
    "esri/Graphic",
    "dojo/domReady!"
], function(
    Map,
    MapView,
    BasemapToggle,
    Point,
    Polyline,
    geometryEngine,
    GraphicsLayer,
    SimpleMarkerSymbol,
    SimpleLineSymbol,
    TextSymbol,
    Graphic,
) {
    var setDepartureICAO = null;
    var routeData        = [];
    var selectedRoute    = null;
    var countryList      = null;

    var map = new Map({
        basemap: "gray-vector"
    });

    var view = new MapView({
        container: "mapDiv",
        map: map
    });

    var basemapToggle = new BasemapToggle({
        view: view,
        nextBasemap: "hybrid"
    });

    var runwayLayer = new GraphicsLayer({
        minScale: 200000
    });
    
    map.add(runwayLayer);
    view.ui.add(basemapToggle, "bottom-right");

    var routeSelectorScrollbar = $("#route-selector #scrollable");

    $("#filters #scrollable").perfectScrollbar();
    routeSelectorScrollbar.perfectScrollbar();
    $("#route-viewer #scrollable").perfectScrollbar();

    // Get country list and add them as autocomplete suggestions
    $.ajax({
        type: 'get',
        url: '/countries',
        success: function(countries) {
            countryList = countries;

            var tags = countries.map(function(country) {
                return country.name;
            });

            var getLast = function(term) {
                return getInsertedCountries(term).pop();
            };

            $("#filters input[name$=_countries]").autocomplete({
                // http://jqueryui.com/autocomplete/#multiple
                source: function(request, response) {
                    var results = $.ui.autocomplete.filter(
                                    tags,
                                    getLast(request.term));

                    response(results.slice(0, 10));
                },
                focus: function() {
                    return false;
                },
                select: function(event, ui) {
                    var terms = getInsertedCountries(this.value);
                    terms.pop();
                    terms.push(ui.item.value);
                    terms.push("");

                    this.value = terms.join(", ");
                    return false;
                }
            });
        },
        error: function(req, errText, err) {
            console.log("error getting country list: " + req.responseText);
        }
    });

    var routeTable = $("#route-selector #route-table");

    // Submit filters and add found routes
    $("#filters").submit(function(e) {
        $.ajax({
            type: 'post',
            url:  '/filter',
            data: serializeFilters(),
            success: function(routes) {
                clearTable(routeTable[0]);
                resetScrollbar(routeSelectorScrollbar);

                routeData = [];

                for(i = 0; i < routes.length; ++i) {
                    insertRoute(routes[i]);
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
    $("#filters input[name=speed]").on("input", function() {
        findButton.prop("disabled", $(this).val() == "");
    });

    // Clear the selected country when the X button is clicked
    $("#filters").on("click", ".clear-country", function() {
        $(this).prev().val('');
    });

    // Highlight route and draw it on the map
    $("#route-table").on("mouseenter", ".route-data", function() {
        $('.highlight').removeClass('highlight');
        $(this).addClass('highlight');

        view.graphics.removeAll();

        var route    = routeData[$(this).index() - 1];
        var depPoint = airportPosToPoint(route.departure.pos);
        var arrPoint = airportPosToPoint(route.arrival.pos);

        drawRoute(depPoint, arrPoint, view);
    });

    // Populate airport info
    $("#route-table").on("click", ".route-data", function() {
        $('.selected').removeClass('selected');
        $(this).addClass('selected');

        resetScrollbar($("#route-viewer #scrollable"));
        selectedRoute = routeData[$(this).index() - 1];

        if(setDepartureICAO != selectedRoute.departure.icao) {
            populateAirportInfo("#route-viewer #departure", selectedRoute.departure);
            setDepartureICAO = selectedRoute.departure.icao;
        }

        populateAirportInfo("#route-viewer #arrival", selectedRoute.arrival);

        runwayLayer.graphics.removeAll();
        displayRunways(selectedRoute.departure.runways);
        displayRunways(selectedRoute.arrival.runways);
    });

    // Teleport map to departure airport
    $("#route-viewer").on("click", "#departure", function() {
        if(!selectedRoute)
            return;

        viewAirport(selectedRoute.departure);
    });

    // Teleport map to arrival airport
    $("#route-viewer").on("click", "#arrival", function() {
        if(!selectedRoute)
            return;
        
        viewAirport(selectedRoute.arrival);
    });

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

        var geodesicLine = geometryEngine.geodesicDensify(linePath, 10000);

        view.graphics.add(new Graphic(startPoint, marker));
        view.graphics.add(new Graphic(endPoint, marker));
        view.graphics.add(new Graphic(geodesicLine, lineSymbol));
    }

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
                var north  = runway.sides.north;
                var south  = runway.sides.south;

                var row = runwayTable.insertRow();
                
                var name = row.insertCell(0);
                name.innerHTML = north.name + " / " + south.name;
                name.className = "data-value";

                var hdgs = row.insertCell(1);
                
                // Although the true heading is a provided field from the
                // airport data source, some airports do not contain it,
                // so it's more reliable to calculate it ourselves
                if(north.pos && south.pos) {
                    var northPoint = latLonToPoint(north.pos);
                    var southPoint = latLonToPoint(south.pos);

                    var northHDG = Math.round(angleFromPoints(northPoint, southPoint));
                    hdgs.innerHTML = northHDG + " / " + ((northHDG + 180) % 360);
                } else {
                    hdgs.innerHTML = "unk / unk";
                }

                hdgs.className = "data-value";

                var length = row.insertCell(2);
                length.innerHTML = runway.length ? runway.length + " ft" : "n/a";
                length.className = "data-value";

                var width = row.insertCell(3);
                width.innerHTML = runway.width ? runway.width + " ft" : "n/a";
                width.className = "data-value";
            }
        }
    }

    function displayRunways(runways) {
        if(!runways)
            return;

        var runwaySymbol = new SimpleLineSymbol({
            width: 3
        });

        for(i = 0; i < runways.length; ++i) {
            var runway = runways[i];

            if(!runway.sides.north.pos || !runway.sides.south.pos)
                continue;

            var northPos = new Point({
                x: runway.sides.north.pos.lon,
                y: runway.sides.north.pos.lat
            });

            var southPos = new Point({
                x: runway.sides.south.pos.lon,
                y: runway.sides.south.pos.lat
            });

            var runwayLine = new Polyline({
                paths: [[northPos.x, northPos.y], [southPos.x, southPos.y]]
            });

            runwayLayer.graphics.add(new Graphic(runwayLine, runwaySymbol));

            var runwayAng = angleFromPoints(northPos, southPos);

            var northText = new TextSymbol({
                color: "black",
                text: runway.sides.north.name,
                angle: runwayAng,
                yoffset: -10,
                font: {
                    size: 8,
                    family: "sans-serif"
                }
            });

            runwayLayer.graphics.add(new Graphic(northPos, northText));

            var southText = northText.clone();
            southText.text = runway.sides.south.name;
            southText.angle += 180;

            runwayLayer.graphics.add(new Graphic(southPos, southText));
        }
    }

    function toRad(ang) {
        return ang * (Math.PI / 180);
    }

    function toDeg(ang) {
        return ang * (180 / Math.PI);
    }

    // https://stackoverflow.com/a/18738281
    function angleFromPoints(pointA, pointB) {
        var pointA = new Point(toRad(pointA.x), toRad(pointA.y));
        var pointB = new Point(toRad(pointB.x), toRad(pointB.y));

        var dLon = pointB.x - pointA.x;

        var y = Math.sin(dLon) * Math.cos(pointB.y);
        var x = Math.cos(pointA.y) * Math.sin(pointB.y) - Math.sin(pointA.y)
                * Math.cos(pointB.y) * Math.cos(dLon);

        return (toDeg(Math.atan2(y, x)) + 360) % 360;
    }

    function clearTable(table) {
        for(i = table.rows.length - 1; i > 0; --i) {
            table.deleteRow(i);
        }
    }

    function resetScrollbar(elem) {
        elem.scrollTop(0);
        elem.perfectScrollbar('update');
    }

    function insertRoute(route) {
        var row = routeTable[0].insertRow();
        row.className = "route-data";
        
        var depRow = row.insertCell(0);
        depRow.innerHTML = route.departure.icao;
        depRow.className = "departure";

        var arrRow = row.insertCell(1);
        arrRow.innerHTML = route.arrival.icao;
        arrRow.className = "arrival";

        var distRow = row.insertCell(2);
        distRow.innerHTML = Math.round(route.distance) + " nm";
        
        var timeRow = row.insertCell(3);
        timeRow.innerHTML = formatRouteTime(route.time);
    }

    function formatRouteTime(time) {
        var hours   = Math.floor(time);
        var minutes = Math.floor((time - hours) * 60);

        return hours + ":" + (minutes < 10 ? "0" + minutes : minutes);
    }

    function airportPosToPoint(pos) {
        return new Point({
            latitude: pos.lat,
            longitude: pos.lon
        });
    }

    function viewAirport(airport) {
        view.goTo({
            center: airportPosToPoint(airport.pos),
            scale: runwayLayer.minScale
        });
    }

    function getInsertedCountries(val) {
        return val.split(/,\s*/);
    }

    // https://stackoverflow.com/a/5075798
    function serializeFilters() {
        var values = $("#filters").serializeArray();
        
        for(i = 0; i < values.length; ++i) {
            var name = values[i].name;

            // Convert the country list input values to their respective codes
            // to make processing easier serverside
            if(name == "dep_countries" || name == "arr_countries") {
                var inserted = getInsertedCountries(values[i].value);
                var regionList = [];

                for(j = 0; j < inserted.length; ++j) {
                    var found = countryList.find(function(country) {
                        return country.name == inserted[j];
                    });

                    if(found) {
                        regionList.push(found.region.code);
                    }
                }

                values[i].value = regionList.join(' ');
            }
        }

        return jQuery.param(values);
    }

    function latLonToPoint(pos) {
        return new Point({
            x: pos.lon,
            y: pos.lat
        });
    }
});