import * as React from 'react';
import Input, { InputChangeEvent } from '../Input';

interface State {
    value: string,
}

class ICAOInput extends React.Component<{}, State> {
    state = {
        value: "",
    };

    private static isValid(icao: string): boolean {
        return icao.allChars((ch) => ch.isDigit() || ch.isAlphanumericUpper());
    }

    private onChange = (event: InputChangeEvent) => {
        const icao = event.target.value.toUpperCase();

        if (!ICAOInput.isValid(icao)) {
            event.preventDefault();
            return;
        }

        this.setState({
            value: icao,
        });
    }

    render() {
        return (
            <Input
                label="ICAO"
                inputClasses="icao-input"
                maxLength={4}
                value={this.state.value}
                onChange={this.onChange}
            />
        );
    }
}

export default ICAOInput;