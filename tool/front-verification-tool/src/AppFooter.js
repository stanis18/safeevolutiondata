import React from 'react';

export const AppFooter = (props) => {

    return (
        <div className="layout-footer">
            <img src={props.layoutColorMode === 'light' ? 'assets/layout/images/logo-dark.jpg' : 'assets/layout/images/logo-dark.jpg'} alt="Logo" height="50" className="mr-2" />
            by
            <span className="font-medium ml-2">Formal Blocks</span>
        </div>
    );
}
