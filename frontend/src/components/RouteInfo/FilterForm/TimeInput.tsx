import * as React from 'react';
import Input, { InputChangeEvent } from './Input';
import '../../../util';
import { Time } from '../../../types/time';

interface Props {
    label: string,
    onChange?: (time: Time | null) => void,
}

interface State {
    value: string,
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

    private static parse(value: string): Time | null {
        const hour_min_split = value.split(':');
        const len = hour_min_split.length;

        switch (len) {
            case 0:
                return { hour: 0, minutes: 0 };
            case 2:
                if (!hour_min_split[0].isDigits() || !hour_min_split[1].isDigits())
                    return null;

                const hour = Number(hour_min_split[0]);
                const minutes = Number(hour_min_split[1]);

                return { hour, minutes };
            default:
                return null;
        }
    }

    private onChange = (event: InputChangeEvent) => {
        const value = event.target.value;

        if (!TimeInput.isValidInput(value)) {
            event.preventDefault();
            return;
        }

        this.setState({ value });
        this.props.onChange?.(TimeInput.parse(value));
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