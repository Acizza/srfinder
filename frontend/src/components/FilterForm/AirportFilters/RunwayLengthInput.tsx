import * as React from 'react';
import Input, { InputChangeEvent } from '../Input';

const enum LengthSelector {
    Equal = "eq",
    GreaterThan = "gt",
    LessThan = "lt",
}

interface State {
    value: string,
    selector: LengthSelector,
}

class RunwayLengthInput extends React.Component<{}, State> {
    state = {
        value: "",
        selector: LengthSelector.Equal,
    };

    private static parse(value: string): LengthSelector | null {
        if (value.length === 0)
            return LengthSelector.Equal;

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

        return type;
    }

    private onChange = (event: InputChangeEvent) => {
        const value = event.target.value;
        const selector = RunwayLengthInput.parse(value);

        if (selector === null) {
            event.preventDefault();
            return;
        }

        this.setState({
            value,
            selector,
        });
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