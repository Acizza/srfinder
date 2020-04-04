import * as React from 'react';
import Input, { InputChangeEvent } from '../Input';

interface Props {
    value: string,
    onChange?: (value: string) => void,
}

class ICAOInput extends React.Component<Props> {
    private static isValid(icao: string): boolean {
        return icao.allChars((ch) => ch.isDigit() || ch.isAlphanumericUpper());
    }

    private onChange = (event: InputChangeEvent) => {
        const icao = event.target.value.toUpperCase();

        if (!ICAOInput.isValid(icao)) {
            event.preventDefault();
            return;
        }

        this.props.onChange?.(icao);
    }

    render() {
        return (
            <Input
                label="ICAO"
                inputClasses="icao-input"
                maxLength={4}
                value={this.props.value}
                onChange={this.onChange}
            />
        );
    }
}

export default ICAOInput;