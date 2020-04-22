import * as React from 'react';
import Input, { InputChangeEvent } from '../Input';

interface Props {
    onChange?: (selectedCountries: string[]) => void,
}

interface State {
    value: string,
}

class CountriesInput extends React.Component<Props, State> {
    state: State = {
        value: "",
    };

    private onChange = (event: InputChangeEvent) => {
        const value = event.target.value;
        this.setState({ value });

        if (!this.props.onChange)
            return;

        const selected = value
            .split(',')
            .reduce((acc, value) => {
                const trimmed = value.trim();

                if (trimmed.length > 0)
                    acc.push(trimmed);

                return acc;
            }, [] as string[]);

        this.props.onChange!(selected);
    }

    render() {
        return (
            <Input
                label="Countries"
                inputClasses="countries-input"
                value={this.state.value}
                onChange={this.onChange}
            />
        );
    }
}

export default CountriesInput;