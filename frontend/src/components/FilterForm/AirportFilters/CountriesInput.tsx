import * as React from 'react';
import Input, { InputChangeEvent } from '../Input';

interface State {
    value: string,
    selected_countries: string[],
}

class CountriesInput extends React.Component<{}, State> {
    state = {
        value: "",
        selected_countries: [],
    };

    private onChange = (event: InputChangeEvent) => {
        const value = event.target.value;
        const selected = value.split(",").map((country: string) => country.trim());

        this.setState({
            value,
            selected_countries: selected,
        });
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