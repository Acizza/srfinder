import * as React from 'react';
import Tabs, { Tab } from './Tabs';
import FilterForm, { SubmitEvent } from './FilterForm/FilterForm';
import RouteViewer from './RouteViewer';
import { Route } from '../../types/route';
import './RouteInfo.css';

interface Props {
    routes: Route[],
    onRoutesRequested?: (filters: any, event: SubmitEvent) => void,
    onRouteSelected?: (route: Route) => void,
}

class RouteInfo extends React.Component<Props> {
    render() {
        const tabs: Tab[] = [
            { name: "filters", content: <FilterForm className="filter-form" onSubmit={this.props.onRoutesRequested} /> },
            { name: "runways", content: <p>TODO (2)</p> },
            { name: "freqs", content: <p>TODO (3)</p> },
        ];

        return (
            <div className="route-info">
                <Tabs tabs={tabs} />
                <RouteViewer routes={this.props.routes} onClick={this.props.onRouteSelected} />
            </div>
        );
    }
}

export default RouteInfo;