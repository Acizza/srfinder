import React from 'react';
import Input from './Input.jsx';
import '../../util.js';

class TimeInput extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            value: "",
            isValidTime: true,
        };
    }

    isValidInput(value) {
        const hour_min_split = value.split(':');
        const len = hour_min_split.length;

        if (len === 0)
            return true;

        if (!this.isValidTimeFragment(hour_min_split[0]))
            return false;

        if (len > 1) {
            if (!this.isValidTimeFragment(hour_min_split[1]) || !this.isValidMinute(hour_min_split[1]))
                return false;
        }

        return len <= 2;
    }

    isValidTime(value) {
        if (value.length === 0)
            return true;

        const hour_min_split = value.split(':');

        if (hour_min_split.length < 2)
            return false;

        return this.isValidTimeFragment(hour_min_split[0]) && this.isValidTimeFragment(hour_min_split[1]);
    }

    isValidTimeFragment(value) {
        return value.length <= 2 && value.isDigits();
    }

    isValidMinute(value) {
        switch (value.length) {
            case 0:
                return true;
            case 1:
                return value[0] <= '5';
            case 2:
                return value[0] <= '5' && value[1] <= '9';
            default:
                return false;
        }
    }

    onChange = (event) => {
        const value = event.target.value;

        if (!this.isValidInput(value)) {
            event.preventDefault();
            return;
        }

        this.setState({
            value: value,
            isValidTime: this.isValidTime(value),
        });
    }

    render() {
        return (
            <Input
                label={this.props.label}
                inputClasses="time-input"
                value={this.state.value}
                onChange={this.onChange}
            />
        );
    }
}

export default TimeInput;