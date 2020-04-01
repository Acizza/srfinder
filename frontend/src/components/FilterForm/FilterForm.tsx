import * as React from 'react';
import AirportFilters from './AirportFilters/AirportFilters';
import SpeedInput, { Speed } from './SpeedInput';
import TimeBox from './TimeBox';
import '../../util';
import './FilterForm.css';

export type FilterFormSubmitEvent = React.FormEvent<HTMLFormElement>;

interface Props {
    onSubmit?: (event: FilterFormSubmitEvent) => void,
    className?: string,
}

interface State {
    speed: Speed,
    departure?: any,
    arrival?: any,
}

class FilterForm extends React.Component<Props, State> {
    state = {
        speed: new Speed(),
    };

    onSpeedChange = (newSpeed: Speed) => this.setState({
        speed: newSpeed,
    });

    render() {
        return (
            <form onSubmit={this.props.onSubmit} className={this.props.className}>
                <SpeedInput speed={this.state.speed} onChange={this.onSpeedChange} />
                <AirportFilters label="Departure" className="departure-filters" />
                <AirportFilters label="Arrival" className="arrival-filters" />
                <TimeBox />
                <input type="submit" className="find-routes-btn" value="Find Routes" />
            </form>
        );
    }
}

export default FilterForm;