# NextJS

## What is NextJS
* NextJs is a framework built on top of React to help building React apps
* In development phase, NextJS brings tools such as ESLint and fast refresh
* For production, NextJS has a compiler (written in Rust) to optimize compilation, minification, bundling and more

## Principles
* Each file in `pages/` folder is an entry
    * File `pages/index.js|ts` is `/`
    * File `pages/posts/first-post.js|ts` is `/posts/first-post`
* Each page is a bundle and shared code form another bundle. It is to optimize loading and load only code needed for a given page
* NextJs propose three rendering modes
    * Server-Side Rendering: each request makes the server build the html and other and serves it. Use `getServerSideProps`
    * Static Site Generation: all is built and there is no need for a server anymore. It is possible to use technics such as CDN. Use `getStaticProps`. It is possible to use incremental site regeneration to update only what changed
    * Client-Side Rendering: method of React for instance. Server sends empty html page and Javascript fill it
* With NextJS, code can be deployed on
    * Origin servers: 
    * CDN: can be placed between client and origin server to cache response
    * Edge: are like CDN but closer to end user (see AWS which propose deploying on Edges)

## Concepts
* `Link` are used for client side navigation, handled by Javascript. `a` are server side navigation elements
* NextJS optimize image rendering with `WebP`(https://developer.mozilla.org/fr/docs/Web/Media/Formats/Image_types#webp)
* CSS Modules (https://nextjs.org/docs/pages/building-your-application/styling) allows to scope CSS at component level
    * CSS Modules automatically generate unique class names
* `_app.js|ts` page is used to apply global things like css. It wraps all application
* NextJS supports Tailwind. To use it, need to customize default PostCSS processor 
```
npm install -D tailwindcss autoprefixer postcss

// postcss.config.js
module.exports = {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
};

// tailwind.config.js
module.exports = {
  content: [
    './pages/**/*.{js,ts,jsx,tsx}',
    './components/**/*.{js,ts,jsx,tsx}',
    // For the best performance and to avoid false positives,
    // be as specific as possible with your content configuration.
  ],
};
```
* To use Sass, simply install it `npm install -D sass`

## Tips
* Use library such as clx (https://www.npmjs.com/package/clsx) to toggle classes
```
CSS
.success {
  color: green;
}
.error {
  color: red;
}

HTML
import styles from './alert.module.css';
import { clsx } from 'clsx';

export default function Alert({ children, type }) {
  return (
    <div
      className={clsx({
        [styles.success]: type === 'success',
        [styles.error]: type === 'error',
      })}
    >
      {children}
    </div>
  );
}
```