import * as React from 'react';
import Input, { InputChangeEvent } from '../Input';

interface Props {
    onChange?: (selectedCountries: string[]) => void,
}

interface State {
    value: string,
}

class CountriesInput extends React.Component<Props, State> {
    state = {
        value: "",
        selectedCountries: [],
    };

    private onChange = (event: InputChangeEvent) => {
        const value = event.target.value;
        this.setState({ value });

        if (!this.props.onChange)
            return;

        const selected = value.split(",").map((country: string) => country.trim());
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