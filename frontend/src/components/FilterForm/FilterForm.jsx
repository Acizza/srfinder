import React from 'react';
import FilterBox from './FilterBox.jsx';
import SpeedInput from './SpeedInput.jsx';
import TimeBox from './TimeBox.jsx';
import '../../util';
import './FilterForm.css';

class FilterForm extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            speed: {},
            departure: null,
            arrival: null,
        };
    }

    onSpeedChange = (state) => this.setState({
        speed: state,
    });

    render() {
        return (
            <form onSubmit={this.props.onSubmit} id={this.props.id}>
                <SpeedInput onChange={this.onSpeedChange} />
                <FilterBox label="Departure" className="departure-filters" />
                <FilterBox label="Arrival" className="arrival-filters" />
                <TimeBox />
                <input type="submit" className="find-routes-btn" value="Find Routes" />
            </form>
        );
    }
}

export default FilterForm;