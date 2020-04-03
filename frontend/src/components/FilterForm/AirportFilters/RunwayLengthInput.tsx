import * as React from 'react';
import Input, { InputChangeEvent } from '../Input';

export const enum LengthSelector {
    Equal = "eq",
    GreaterThan = "gt",
    LessThan = "lt",
}

export interface RunwayLength {
    length: number,
    selector: LengthSelector,
}

interface Props {
    onChange?: (length: RunwayLength) => void,
}

interface State {
    value: string,
    selector: LengthSelector,
}

class RunwayLengthInput extends React.Component<Props, State> {
    state = {
        value: "",
        selector: LengthSelector.Equal,
    };

    private static parse(value: string): [number, LengthSelector] | null {
        if (value.length === 0)
            return [0, LengthSelector.GreaterThan];

        let type: LengthSelector;
        let slice: string;

        switch (value[0]) {
            case '>':
                type = LengthSelector.GreaterThan;
                slice = value.substr(1);
                break;
            case '<':
                type = LengthSelector.LessThan;
                slice = value.substr(1);
                break;
            default:
                type = LengthSelector.Equal;
                slice = value;
                break;
        }

        if (!slice.isDigits())
            return null;

        return [Number(slice), type];
    }

    private onChange = (event: InputChangeEvent) => {
        const value = event.target.value;
        const result = RunwayLengthInput.parse(value);

        if (result === null) {
            event.preventDefault();
            return;
        }

        const [length, selector] = result;

        this.setState({
            value,
            selector,
        });

        this.props.onChange?.({ length, selector });
    };

    render() {
        return (
            <Input
                label="Length"
                inputClasses="length-input"
                value={this.state.value}
                maxLength={6}
                onChange={this.onChange}
            />
        );
    }
}

export default RunwayLengthInput;