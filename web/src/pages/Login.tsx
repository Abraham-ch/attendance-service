import Backdrop from '../components/atoms/Backdrop'
import Heading from '../components/atoms/Heading'
import Label from '../components/atoms/Label'
import Brand from '../components/molecules/Brand'
import LoginForm from '../components/organisms/LoginForm'

function Login() {
  return (
    <main className="grid min-h-svh lg:grid-cols-2">
      <section className="relative isolate hidden flex-col justify-between border-r border-line p-edge lg:flex">
        <Backdrop />
        <Brand />
        <Heading size="title">
          Back to <span className="block text-brass">the roll</span>
        </Heading>
        <Label>Roll call in real time</Label>
      </section>
      <section className="flex items-center justify-center p-edge">
        <LoginForm />
      </section>
    </main>
  )
}

export default Login
