import React from 'react';
import { setDefaultOptions, loadModules } from 'esri-loader';
import './RouteMap.css';

class RouteMap extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            isLoaded: false
        };

        setDefaultOptions({ css: true });

        loadModules(["esri/Map", "esri/views/MapView", "esri/widgets/BasemapToggle"]).then(([Map, MapView, BasemapToggle]) => {
            const map = new Map({
                basemap: "gray-vector"
            });

            const view = new MapView({
                container: "route-map-container",
                map: map,
                zoom: 2
            });

            const basemapToggle = new BasemapToggle({
                view: view,
                nextBasemap: "hybrid"
            });

            view.ui.add(basemapToggle, "bottom-right");

            this.setState({
                map: map,
                view: view,
                isLoaded: true,
            });
        }).catch(err => {
            console.error(err);
        });
    }

    render() {
        return (
            <React.Fragment>
                {!this.state.isLoaded && <h2 id="loading-text">Loading map</h2>}
                <div id="route-map-container"></div>
            </React.Fragment>
        );
    }
}

export default RouteMap;