import * as React from 'react';
import AirportFilters, { State as AirportFiltersState } from './AirportFilters/AirportFilters';
import SpeedInput, { Speed } from './SpeedInput';
import TimeRangeBox from './TimeRangeBox';
import { TimeRange } from '../../../types/time';
import '../../../util';
import './FilterForm.css';

export type SubmitEvent = React.FormEvent<HTMLFormElement>;

interface Props {
    onSubmit?: (state: State, event: SubmitEvent) => void,
    className?: string,
}

export interface State {
    speed: Speed,
    departure?: AirportFiltersState,
    arrival?: AirportFiltersState,
    timeRange?: TimeRange,
}

class FilterForm extends React.Component<Props, State> {
    state: State = {
        speed: new Speed(),
    };

    private onAirportFiltersChange = (type: "departure" | "arrival", value: AirportFiltersState) => {
        switch (type) {
            case "departure":
                this.setState({ departure: value });
                break;
            case "arrival":
                this.setState({ arrival: value });
                break;
        }
    }

    render() {
        return (
            <form onSubmit={event => this.props.onSubmit?.(this.state, event)} className={this.props.className}>
                <SpeedInput speed={this.state.speed} onChange={speed => this.setState({ speed })} />
                <AirportFilters label="Departure" className="departure-filters" onChange={value => this.onAirportFiltersChange("departure", value)} />
                <AirportFilters label="Arrival" className="arrival-filters" onChange={value => this.onAirportFiltersChange("arrival", value)} />
                <TimeRangeBox onChange={timeRange => this.setState({ timeRange })} />
                <input type="submit" className="find-routes-btn" value="Find Routes" />
            </form>
        );
    }
}

export default FilterForm;