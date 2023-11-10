# Passport JS

## What is PassportJS?
* Javascript library to handle various authentication mechanism.

## User session, Serialize & Deserialize user
* https://www.passportjs.org/concepts/authentication/sessions/
* To maintain session, Passport can use sessions defined by server like with Express
```
app.use(session({
  secret: 'keyboard cat',
  resave: false,
  saveUninitialized: false,
  cookie: { secure: true }
}));
```
* To save user information, Passport serialize user and deserialize in session store
```
  passport.serializeUser(function (user, done) {
    done(null, user); // May be done(null, user.id)
  });
  passport.deserializeUser(function (user: any, done) {
    done(null, user); // Retrieve user info from somewhere
  });
```
* When deserializing user from session during a request, the user information are attached to the request `req.user`
* User is stored as `req.session.passport.user = <second argument of done callback>`
* Parameter received in `passport.serializeUser` comes from used strategy
```
passport.use('oidc', new Strategy({ client: keycloakClient }, (tokenSet: any, userinfo: any, done: any) => {
      // tokenSet will be the parameter of serializeUser
      return done(null, tokenSet);
    })
  );
```