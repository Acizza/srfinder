import * as React from 'react';
import Input, { InputChangeEvent } from './Input';

interface Props {
    speed: Speed,
    onChange?: (newSpeed: Speed) => void;
}

interface State {
    speed: Speed,
}

class SpeedInput extends React.Component<Props, State> {
    state = {
        speed: this.props.speed,
    };

    private onChange = (event: InputChangeEvent) => {
        const speed = Speed.fromString(event.target.value);

        if (speed === null) {
            event.preventDefault();
            return;
        }

        this.setState({
            speed: speed,
        });

        this.props.onChange?.(speed);
    }

    render() {
        return (
            <Input
                label="Cruise Speed"
                labelClasses="speed-label"
                inputClasses="speed-input"
                value={this.state.speed.value.toString()}
                onChange={this.onChange}
            />
        );
    }
}

export const enum SpeedType {
    Mach = "mach",
    Knots = "knots",
}

export class Speed {
    constructor(public value = "0.770", public type = SpeedType.Mach) { }

    static fromString(value: string): Speed | null {
        if (!Speed.isValid(value))
            return null;

        const num = parseFloat(value);
        const type = Speed.isWholeNumber(num) ? SpeedType.Knots : SpeedType.Mach;

        return new Speed(value, type);
    }

    private static isValid(value: string): boolean {
        const split = value.split('.');

        switch (split.length) {
            case 0:
                return value.isDigits();
            case 1:
                return split[0].isDigits();
            case 2:
                return split[0].isDigits() && split[1].isDigits();
            default:
                return false;
        }
    }

    private static isWholeNumber(num: number): boolean {
        return num % 1 === 0;
    }
}

export default SpeedInput;