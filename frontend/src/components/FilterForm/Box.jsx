import React from 'react';

function Box(props) {
    return (
        <div className={"box " + props.className}>
            <label>{props.label}</label>
            <div className="box-content">
                {props.children}
            </div>
        </div>
    );
}

export default Box;