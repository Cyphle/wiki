# Spring Security

## Security internal flow
1. User enters credentials, send to spring security filters
2. If needed, authentication. Create a session. Filters extract username and store in authentication object
3. Filters hand over security to authentication manager. Will check what providers are presents to actually authenticate (from LDAP, OAuth, etc)
4. Authentication providers use UserDetailsManger/service to extract user details and check password with pasword encoder
5. Then security filters store authentication object in security context

## Password encoder
Pour définir et utiliser un password encoder, soit on utilise celui par défaut par exemple
```
User
                .withDefaultPasswordEncoder()
                .username("user")
                .password("{noop}password")
                .authorities("read")
                .build()
```
soit on définit un bean définissant le password encoder
```
@Bean
fun passwordEncoder(): PasswordEncoder {

}
```
Si on définit son propre password encoder, Spring va l'utiliser par défaut.

## Authentication provider
Un authentication provider permet de définir la méthode d'authentification (OAuth, OTP : one time password, etc). Avec Spring, il est possible d'avoir plusieurs authentication provider pour accepter plusieurs modes d'authentification.