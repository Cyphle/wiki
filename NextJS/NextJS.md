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
* NextJS pre-render every page in advance. It generates HTML in advance
    * It is possible to mix server side rendering and static site rendering
    * Note that server side rendering is slower but a wiser choice when a page and conditionned by user requests
* For prerendering pages with data
    * Can use `getStaticProps` which is run at build time and tell NextJS to fetch data when prerendering the page. As the name says it, use it for static data not when depending on user requests. The method has to be in each page that uses it and only in pages.
    * For data using server side depending on request, use `getServerSideProps`
```
export async function getServerSideProps(context) { // Context contain requested parameters
  return {
    props: {
      // props for your component
    },
  };
}
```
    * For other case, use normal strategy using client side rendering. It is still possible to optimize the requests with caching and other technics. Use `SWL`(https://swr.vercel.app/fr-FR) hook, it is a helper for these purposes built by NextJS team.
```
import useSWR from 'swr';

function Profile() {
  const { data, error } = useSWR('/api/user', fetch);

  if (error) return <div>failed to load</div>;
  if (!data) return <div>loading...</div>;
  return <div>hello {data.name}!</div>;
}
```
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

## Routing
* For dynamic routing, have to implement a function that list the paths and a funciton to get data depending on path
```
export async function getStaticPaths() {
  const paths = getAllPostIds();
  return {
    paths,
    fallback: false,
  };
}

export async function getStaticProps({ params }) {
  // Add the "await" keyword like this:
  const postData = await getPostData(params.id);

  return {
    props: {
      postData,
    },
  };
}
```
* `getStaticPaths` runs at build time in production, for each request in dev
* `fallback: true` in `getStaticPaths` means that if there is a 404, it will fallback to a page created by NextJS. See https://nextjs.org/docs/pages/api-reference/functions/get-static-paths#fallback-false

## API Routes
* NextJS allows to create API routes with files in `pages/api` and exporting `export default function handler(req:  http.IncomingMessage, res: http.ServerResponse) { }`
* You should not fetch an API Route from `getStaticProps` or `getStaticPaths`. Instead, write your server-side code directly in `getStaticProps` or `getStaticPaths` (or call a helper function). See https://nextjs.org/docs/pages/building-your-application/data-fetching/get-static-props#write-server-side-code-directly
* API Routes will not be part of client code.

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