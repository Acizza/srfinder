import * as React from 'react';

// Each property here must match its equivalent backend airport type.
interface AirportTypes {
    unknown: string,
    large_airport: string,
    medium_airport: string,
    small_airport: string,
    closed: string,
    heliport: string,
    seaplane_base: string,
}

export type AirportType = keyof AirportTypes;

interface Props {
    selected: AirportType,
    onChange?: (selected: AirportType) => void,
}

class AirportTypeInput extends React.Component<Props> {
    static readonly typeNames: AirportTypes = {
        unknown: "",
        large_airport: "Large",
        medium_airport: "Medium",
        small_airport: "Small",
        closed: "Closed",
        heliport: "Heliport",
        seaplane_base: "Seaplane Base",
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