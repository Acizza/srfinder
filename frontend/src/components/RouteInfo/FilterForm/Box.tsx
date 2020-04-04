import * as React from 'react';
import './Box.css';

interface Props {
    label: string,
    className?: string,
    children?: any,
}

function Box(props: Props): JSX.Element {
    return (
        <div className={`box ${props.className}`}>
            <label>{props.label}</label>
            <div className="box-content">
                {props.children}
            </div>
        </div>
    );
}

export default Box;