import * as React from 'react';

interface AirportTypes {
    none: string,
    large: string,
    medium: string,
    small: string,
}

export type AirportType = keyof AirportTypes;

interface Props {
    selected: AirportType,
    onChange?: (selected: AirportType) => void,
}

class AirportTypeInput extends React.Component<Props> {
    static readonly typeNames: AirportTypes = {
        none: "",
        large: "Large",
        medium: "Medium",
        small: "Small",
    };

    static readonly types = Object.keys(AirportTypeInput.typeNames).map(type => {
        const display = (AirportTypeInput.typeNames as any)[type];

        return (
            <option key={type} value={type}>{display}</option>
        );
    });

    private onChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
        const type = event.target.value as AirportType;
        this.props.onChange?.(type);
    }

    render() {
        return (
            <React.Fragment>
                <label>Type</label>
                <select className="airport-type-input" value={this.props.selected} onChange={this.onChange}>
                    {AirportTypeInput.types}
                </select>
            </React.Fragment>
        );
    }
}

export default AirportTypeInput;