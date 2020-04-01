import * as React from 'react';

enum AirportType {
    None = "",
    Large = "Large",
    Medium = "Medium",
    Small = "Small",
}

class AirportTypeInput extends React.Component {
    static readonly types = Object.values(AirportType).map((type: string, i) =>
        <option key={i} value={type}>{type}</option>
    );

    render() {
        return (
            <React.Fragment>
                <label>Type</label>
                <select className="airport-type-input">
                    {AirportTypeInput.types}
                </select>
            </React.Fragment>
        );
    }
}

export default AirportTypeInput;