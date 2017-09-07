import React from 'react';
import ReactDOM from 'react-dom';
import { BrowserRouter, Route, Switch } from 'react-router-dom';
import { Tabs } from './tabs';

const API_URL = 'http://127.0.0.1:3000/data/';

const App = (props) => {
  let body;
  if (props.data) {
    body = (<Switch>
      <Route exact path='/' render={routeProps => <Tabs {...routeProps} tabs={props.data.tabs}/>} />
      <Route exact path='/:tab/:category' render={routeProps => <Tabs {...routeProps} tabs={props.data.tabs}/>} />
      <Route exact path='/:tab' render={routeProps => <Tabs {...routeProps} tabs={props.data.tabs}/>} />
    </Switch>);
  } else {
    body = <div>loading...</div>
  }
  return (
    <BrowserRouter>
      <div>
        <img className="logo" src="/static/rust-logo-256x256-blk.png" height="128" width="128" alt="Rust logo" />
        <div className="header">
          <h2>Find something Rusty to work on!</h2>
        </div>
        <div className="clear"></div>
        <p className="pitch narrow">
            Are you fast, friendly, and fearless? You might find fun fixing Rust!
        </p>
        <p className="narrow">
            TODO what is Rust? some more text about things to work on
        </p>

        {body}

        <p className="footer">
        </p>
      </div>
    </BrowserRouter>
  );
};

export function renderApp() {
    const container = document.getElementById('container')
    ReactDOM.render(
        <App />,
        container
    );

    fetch(API_URL).then(function(response) {
        return response.json();
    }).then(function(data) {
        ReactDOM.render(
            <App data={data}/>,
            container
        );
    });
}
