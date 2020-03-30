import * as React from 'react';
import Input, { InputChangeEvent } from './Input';
import '../../util';

interface Props {
    label: string,
}

interface State {
    value: string,
    isValidTime: boolean,
}

class TimeInput extends React.Component<Props, State> {
    state = {
        value: "",
        isValidTime: true,
    };

    private static isValidInput(value: string): boolean {
        const hour_min_split = value.split(':');
        const len = hour_min_split.length;

        if (len === 0)
            return true;

        if (!this.isValidTimeFragment(hour_min_split[0]))
            return false;

        if (len > 1) {
            if (!this.isValidTimeFragment(hour_min_split[1]) || !TimeInput.isValidMinute(hour_min_split[1]))
                return false;
        }

        return len <= 2;
    }

    private static isValidTime(value: string): boolean {
        if (value.length === 0)
            return true;

        const hour_min_split = value.split(':');

        if (hour_min_split.length < 2)
            return false;

        return this.isValidTimeFragment(hour_min_split[0]) && this.isValidTimeFragment(hour_min_split[1]);
    }

    private static isValidTimeFragment(value: string): boolean {
        return value.length <= 2 && value.isDigits();
    }

    private static isValidMinute(value: string): boolean {
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

    private onChange = (event: InputChangeEvent) => {
        const value = event.target.value;

        if (!TimeInput.isValidInput(value)) {
            event.preventDefault();
            return;
        }

        this.setState({
            value: value,
            isValidTime: TimeInput.isValidTime(value),
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