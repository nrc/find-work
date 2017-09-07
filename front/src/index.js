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
            <a href="https://www.rust-lang.org" target="_blank">Rust</a> is a systems programming
            language that runs blazingly fast, prevents segfaults, and guarantees thread safety.
        </p>
        <p className="narrow">
            Rust has a friendly community and lots of interesting, high impact problems to solve. We
            love new contributors and there are many experienced community members happy to mentor
            you. This is the place to find something to work on - in the core language or the wider
            ecosystem.
        </p>
        <p className="narrow">
            There is <a href="https://www.rust-lang.org/en-US/contribute.html" target="_blank">more
            information about contributing</a> on the Rust website.
        </p>
        <p className="narrow">
            We pride ourselves on maintaining civilized discourse, and to that end contributors are
            expected to follow our <a href="https://www.rust-lang.org/conduct.html" target="_blank">Code of Conduct</a>.
        </p>

        {body}

        <p className="footer">
            Found a bug with this website? Want to contribute? <a href="https://github.com/nrc/find-work" target="_blank">Visit the repo</a>.
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
