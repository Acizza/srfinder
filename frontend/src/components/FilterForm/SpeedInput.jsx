import React from 'react';
import Input from './Input.jsx';

class SpeedInput extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            value: "0.770",
            type: "mach",
        };
    }

    isWholeNumber(num) {
        return num % 1 === 0;
    }

    isNumber(str) {
        return str.allChars((ch) => ch.isDigit() || ch === '.');
    }

    onChange = (event) => {
        const speed = event.target.value;

        if (!this.isNumber(speed)) {
            event.preventDefault();
            return;
        }

        const speed_type = this.isWholeNumber(speed) ? "knots" : "mach";

        const newState = {
            value: speed,
            type: speed_type
        };

        this.setState(newState);
        this.props.onChange(newState);
    }

    render() {
        return (
            <Input
                label="Cruise Speed"
                labelClasses="speed-label"
                inputClasses="speed-input"
                value={this.state.value}
                onChange={this.onChange}
            />
        );
    }
}

export default SpeedInput;