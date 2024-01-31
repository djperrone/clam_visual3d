using Clam;
using UnityEngine;
#if ENABLE_INPUT_SYSTEM
using UnityEngine.InputSystem;
using UnityEngine.UIElements;
#endif

namespace StarterAssets
{
    [RequireComponent(typeof(CharacterController))]
#if ENABLE_INPUT_SYSTEM
    [RequireComponent(typeof(PlayerInput))]
#endif
    public class FirstPersonController : MonoBehaviour
    {
        [Header("Player")]
        [Tooltip("Move speed of the character in m/s")]
        public float MoveSpeed = 100.0f;
        public float scalar = 10.5f;
        [Tooltip("Sprint speed of the character in m/s")]
        public float SprintSpeed = 302.0f;
        [Tooltip("Rotation speed of the character")]
        public float RotationSpeed = 1.0f;
        [Tooltip("Acceleration and deceleration")]
        public float SpeedChangeRate = 10.0f;

        [Space(10)]
        [Tooltip("The height the player can jump")]
        public float JumpHeight = 1.2f;
        [Tooltip("The character uses its own gravity value. The engine default is -9.81f")]
        public float Gravity = -15.0f;

        [Space(10)]
        [Tooltip("Time required to pass before being able to jump again. Set to 0f to instantly jump again")]
        public float JumpTimeout = 0.1f;
        [Tooltip("Time required to pass before entering the fall state. Useful for walking down stairs")]
        public float FallTimeout = 0.15f;

        [Header("Player Grounded")]
        [Tooltip("If the character is grounded or not. Not part of the CharacterController built in grounded check")]
        public bool Grounded = true;
        [Tooltip("Useful for rough ground")]
        public float GroundedOffset = -0.14f;
        [Tooltip("The radius of the grounded check. Should match the radius of the CharacterController")]
        public float GroundedRadius = 0.5f;
        [Tooltip("What layers the character uses as ground")]
        public LayerMask GroundLayers;

        [Header("Cinemachine")]
        [Tooltip("The follow target set in the Cinemachine Virtual Camera that the camera will follow")]
        public GameObject CinemachineCameraTarget;
        [Tooltip("How far in degrees can you move the camera up")]
        public float TopClamp = 90.0f;
        [Tooltip("How far in degrees can you move the camera down")]
        public float BottomClamp = -90.0f;

        // cinemachine
        private float m_CinemachineTargetPitch;

        // player
        private float m_Speed;
        private float m_RotationVelocity;
        private float m_VerticalVelocity;
        private float m_TerminalVelocity = 53.0f;

        // timeout deltatime
        private float m_JumpTimeoutDelta;
        private float m_FallTimeoutDelta;


#if ENABLE_INPUT_SYSTEM
        private PlayerInput m_PlayerInput;
#endif
        private CharacterController m_Controller;
        private StarterAssetsInputs m_Input;
        private GameObject m_MainCamera;

        private const float m_Threshold = 0.01f;

        private bool IsCurrentDeviceMouse
        {
            get
            {
#if ENABLE_INPUT_SYSTEM
                return m_PlayerInput.currentControlScheme == "KeyboardMouse";
#else
				return false;
#endif
            }
        }

        private void Awake()
        {
            // get a reference to our main camera
            if (m_MainCamera == null)
            {
                m_MainCamera = GameObject.FindGameObjectWithTag("MainCamera");
            }
        }

        private void Start()
        {
            m_Controller = GetComponent<CharacterController>();
            m_Input = GetComponent<StarterAssetsInputs>();
#if ENABLE_INPUT_SYSTEM
            m_PlayerInput = GetComponent<PlayerInput>();
#else
			Debug.LogError( "Starter Assets package is missing dependencies. Please use Tools/Starter Assets/Reinstall Dependencies to fix it");
#endif

            // reset our timeouts on start
            m_JumpTimeoutDelta = JumpTimeout;
            m_FallTimeoutDelta = FallTimeout;
        }

        private void Update()
        {
            Move();
        }

        private void LateUpdate()
        {
            CameraRotation();
        }

        private void GroundedCheck()
        {
            // set sphere position, with offset
            Vector3 spherePosition = new Vector3(transform.position.x, transform.position.y - GroundedOffset, transform.position.z);
            Grounded = Physics.CheckSphere(spherePosition, GroundedRadius, GroundLayers, QueryTriggerInteraction.Ignore);
        }

        private void CameraRotation()
        {
            // if there is an input
            if (m_Input.look.sqrMagnitude >= m_Threshold)
            {
                //Don't multiply mouse input by Time.deltaTime
                float deltaTimeMultiplier = IsCurrentDeviceMouse ? 1.0f : Time.deltaTime;

                m_CinemachineTargetPitch += m_Input.look.y * RotationSpeed * deltaTimeMultiplier;
                m_RotationVelocity = m_Input.look.x * RotationSpeed * deltaTimeMultiplier;

                // clamp our pitch rotation
                m_CinemachineTargetPitch = ClampAngle(m_CinemachineTargetPitch, BottomClamp, TopClamp);

                // Update Cinemachine camera target pitch
                CinemachineCameraTarget.transform.localRotation = Quaternion.Euler(m_CinemachineTargetPitch, 0.0f, 0.0f);

                // rotate the player left and right
                transform.Rotate(Vector3.up * m_RotationVelocity);
            }
        }

        private void Move()
        {
            // set target speed based on move speed, sprint speed and if sprint is pressed
            float targetSpeed = m_Input.sprint ? SprintSpeed : MoveSpeed;

            // a simplistic acceleration and deceleration designed to be easy to remove, replace, or iterate upon

            // note: Vector2's == operator uses approximation so is not floating point error prone, and is cheaper than magnitude
            // if there is no input, set the target speed to 0
            if (m_Input.move == Vector2.zero && m_Input.verticalMove == Vector2.zero) targetSpeed = 0.0f;

            // a reference to the players current horizontal velocity
            float currentHorizontalSpeed = new Vector3(m_Controller.velocity.x, m_Controller.velocity.y, m_Controller.velocity.z).magnitude;

            float speedOffset = 0.1f;
            float inputMagnitude = m_Input.analogMovement ? m_Input.move.magnitude : 1f;

            // accelerate or decelerate to target speed
            if (currentHorizontalSpeed < targetSpeed - speedOffset || currentHorizontalSpeed > targetSpeed + speedOffset)
            {
                // creates curved result rather than a linear one giving a more organic speed change
                // note T in Lerp is clamped, so we don't need to clamp our speed
                m_Speed = Mathf.Lerp(currentHorizontalSpeed, targetSpeed * inputMagnitude, Time.deltaTime * SpeedChangeRate);

                // round speed to 3 decimal places
                m_Speed = Mathf.Round(m_Speed * 1000f) / 1000f;
            }
            else
            {
                m_Speed = targetSpeed;
            }
            m_Speed = targetSpeed;


            // normalise input direction
            Vector3 inputDirection = new Vector3(m_Input.move.x, m_Input.verticalMove.y, m_Input.move.y).normalized;

            // note: Vector2's != operator uses approximation so is not floating point error prone, and is cheaper than magnitude
            // if there is a move input rotate player when the player is moving
            if (m_Input.move != Vector2.zero || m_Input.verticalMove != Vector2.zero)
            {
                // move
                inputDirection = transform.right * m_Input.move.x + transform.forward * m_Input.move.y + transform.up * -m_Input.verticalMove.y;
            }

            // move the player
            Vector3 movement = inputDirection.normalized * (m_Speed * Time.deltaTime) + new Vector3(0.0f, m_VerticalVelocity, 0.0f) * Time.deltaTime;
            
            movement.x *= scalar;
            movement.y *= scalar;
            movement.z *= scalar;
            m_Controller.Move(movement);
        }

        void OnExit()
        {
            //Application.Quit();
            m_PlayerInput.SwitchCurrentActionMap("WorldUI");

            MenuEventManager.SwitchState(Menu.Pause);
        }

        public void OnChangeMapToWorldUI(InputValue value)
        {
            m_PlayerInput.SwitchCurrentActionMap("WorldUI");
            Clam.MenuEventManager.SwitchState(Menu.Unlock);
        }

        private void VerticalMove()
        {
            float targetSpeed = m_Input.sprint ? SprintSpeed : MoveSpeed;
            if (m_Input.verticalMove == Vector2.zero) targetSpeed = 0.0f;

            // a reference to the players current horizontal velocity
            float currentHorizontalSpeed = new Vector3(0.0f, m_Controller.velocity.y, 0.0f).magnitude;

            float speedOffset = 0.1f;
            float inputMagnitude = m_Input.analogMovement ? m_Input.verticalMove.magnitude : 1f;

            // accelerate or decelerate to target speed
            if (currentHorizontalSpeed < targetSpeed - speedOffset || currentHorizontalSpeed > targetSpeed + speedOffset)
            {
                // creates curved result rather than a linear one giving a more organic speed change
                // note T in Lerp is clamped, so we don't need to clamp our speed
                m_Speed = Mathf.Lerp(currentHorizontalSpeed, targetSpeed * inputMagnitude, Time.deltaTime * SpeedChangeRate);

                // round speed to 3 decimal places
                m_Speed = Mathf.Round(m_Speed * 1000f) / 1000f;
            }
            else
            {
                m_Speed = targetSpeed;
            }
            
            // normalise input direction
            Vector3 inputDirection = new Vector3(0.0f, m_Input.verticalMove.y, 0.0f).normalized;

            // note: Vector2's != operator uses approximation so is not floating point error prone, and is cheaper than magnitude
            // if there is a move input rotate player when the player is moving
            if (m_Input.verticalMove != Vector2.zero)
            {
                // move
                inputDirection = transform.up * -m_Input.verticalMove.y;
            }

            // move the player
            m_Controller.Move(inputDirection.normalized * (m_Speed * Time.deltaTime) + new Vector3(0.0f, m_VerticalVelocity, 0.0f) * Time.deltaTime);
        }

        private void JumpAndGravity()
        {
            if (Grounded)
            {
                // reset the fall timeout timer
                m_FallTimeoutDelta = FallTimeout;

                // stop our velocity dropping infinitely when grounded
                if (m_VerticalVelocity < 0.0f)
                {
                    m_VerticalVelocity = -2f;
                }

                // Jump
                if (m_Input.jump && m_JumpTimeoutDelta <= 0.0f)
                {
                    // the square root of H * -2 * G = how much velocity needed to reach desired height
                    m_VerticalVelocity = Mathf.Sqrt(JumpHeight * -2f * Gravity);
                }

                // jump timeout
                if (m_JumpTimeoutDelta >= 0.0f)
                {
                    m_JumpTimeoutDelta -= Time.deltaTime;
                }
            }
            else
            {
                // reset the jump timeout timer
                m_JumpTimeoutDelta = JumpTimeout;

                // fall timeout
                if (m_FallTimeoutDelta >= 0.0f)
                {
                    m_FallTimeoutDelta -= Time.deltaTime;
                }

                // if we are not grounded, do not jump
                m_Input.jump = false;
            }

            // apply gravity over time if under terminal (multiply by delta time twice to linearly speed up over time)
            if (m_VerticalVelocity < m_TerminalVelocity)
            {
                m_VerticalVelocity += Gravity * Time.deltaTime;
            }
        }

        private static float ClampAngle(float lfAngle, float lfMin, float lfMax)
        {
            if (lfAngle < -360f) lfAngle += 360f;
            if (lfAngle > 360f) lfAngle -= 360f;
            return Mathf.Clamp(lfAngle, lfMin, lfMax);
        }

        private void OnDrawGizmosSelected()
        {
            Color transparentGreen = new Color(0.0f, 1.0f, 0.0f, 0.35f);
            Color transparentRed = new Color(1.0f, 0.0f, 0.0f, 0.35f);

            if (Grounded) Gizmos.color = transparentGreen;
            else Gizmos.color = transparentRed;

            // when selected, draw a gizmo in the position of, and matching radius of, the grounded collider
            Gizmos.DrawSphere(new Vector3(transform.position.x, transform.position.y - GroundedOffset, transform.position.z), GroundedRadius);
        }
    }
}
