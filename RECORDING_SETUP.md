# ZEclipse Video Demo - Recording & Technical Setup

## Pre-Recording Checklist (30 mins before recording)

### Hardware & Software
- [ ] Microphone tested and working
- [ ] Video capture software installed (OBS Studio recommended, it's free)
- [ ] Screen resolution set to 1920×1080 or higher
- [ ] Close all unnecessary apps (Slack, emails, notifications off)
- [ ] Have a glass of water nearby

### Code Setup
- [ ] Terminal is clean (no old output)
- [ ] Run: `npm install` (should be cached, fast)
- [ ] Test: `npm run demo:video` once to ensure it works
- [ ] Have both files open for reference:
  - `VIDEO_SCRIPT.md` (for segment timing)
  - `VIDEO_TALKING_POINTS.md` (for what to say)

### Terminal Configuration
```bash
# In your terminal, run:

# 1. Clean terminal
clear

# 2. Set good font size (terminal should show ~100 chars per line)
# On macOS Terminal/iTerm2: Command + Mouse drag to resize
# Font should be at least 14-16pt

# 3. Set appropriate colors (dark background, light text)
# Recommended: One Dark Pro or Dracula theme

# 4. Verify demo runs clean
cd /Users/jefferson/Desktop/larp-core/ZEclipse/app
npm run demo:video

# If any errors, fix them BEFORE recording
```

### OBS Studio Setup (Recommended for clean recording)

1. **Install OBS:** https://obsproject.com/
2. **Create new scene:**
   - Source: Display Capture (select your main monitor)
   - Resolution: 1920×1080 @ 30fps
   - Encoder: H.264 (CPU friendly)

3. **Audio:**
   - Input: Your microphone
   - Audio bitrate: 128kbps
   - Check levels: -6db to -3db (not peaking)

4. **Output location:**
   ```
   /Users/jefferson/Desktop/larp-core/ZEclipse/recordings/
   ```
   Create this folder first:
   ```bash
   mkdir -p /Users/jefferson/Desktop/larp-core/ZEclipse/recordings
   ```

### Example OBS Settings
```
Output Resolution: 1920×1080
FPS: 30
Bitrate: 5000 kbps (good quality, smaller file)
Encoder: H.264
Preset: Medium (balance quality/speed)
```

---

## Recording Flow (Step by Step)

### STEP 1: Start Recording (0:00)
```bash
# In OBS Studio, click "Start Recording"
# Verify red circle appears in top-right
```

### STEP 2: Opening Remarks (0:00 - 0:30)
**Speak naturally, don't read script word-for-word:**

> "Hey everyone. Today I want to show you a privacy problem that exists on every blockchain, and a solution called ZEclipse that breaks that problem."

> "We're going to trace a simple transaction: Alice sending 1 SOL to Bob. First, the normal way—which is completely traceable. Then the private way."

**Timing:** ~30 seconds

### STEP 3: Run Demo Segment 1
```bash
npm run demo:video
```

**This outputs:** Segment 1: The Problem

**Point at the output and narrate** (don't just read):
- Point at "Sender: Alice (5x8Hs1...)"
- Point at "Recipient: Bob (H4Kx2...)"
- Point at the direct arrow: `5x8Hs1... ──> H4Kx2...`

**Say (don't script-read):**
> "Here's a direct Solana transfer. This is what everyone sees: Alice's address, Bob's address, the amount, and the timestamp. This is permanently on the blockchain. Forever."

> "The problem? Anyone can trace this. If you know Alice is an exchange, you can see everyone who withdraws from them. If you know Bob is a privacy advocate, you can see who's sending him money."

**Point at privacy score:** "Privacy score: 0 out of 100. That's the problem."

**Timing:** ~1:30-2:00 min

### STEP 4: Show Direct Transfer Code
**Zoom in on `src/demos/demo-video.ts` lines 46-68 in an editor**

Have VSCode open with this visible:
```typescript
async function directTransfer(
  sender: Keypair,
  recipient: PublicKey,
  amount: number
) {
  const transaction = new Transaction().add(
    SystemProgram.transfer({
      fromPubkey: sender.publicKey,    // ✅ Visible on chain
      toPubkey: recipient,              // ✅ Visible on chain
      lamports: amount                   // ✅ Visible on chain
    })
  );
```

**Point and narrate:**
- Highlight `fromPubkey: sender.publicKey`
  > "This is Alice's address. Visible on chain."
- Highlight `toPubkey: recipient`
  > "This is Bob's address. Visible on chain."
- Highlight `lamports: amount`
  > "The amount she sent. Visible on chain."

> "Three pieces of information, permanently linked. That's how blockchains work. It's a feature—they're transparent. But it's a privacy nightmare."

**Timing:** ~1:30-2:00 min

### STEP 5: Run Demo Segment 2-3
**Switch back to terminal** where demo is running

**The demo already shows Segment 2 and 3 in sequence. Let it run.**

**Narrate as it shows:**

**Segment 2 (Code):**
> "Here's the code showing the problem. Notice the visibility comments—everything is on-chain."

**Segment 3 (Solution):**
> "Now here's the solution. Instead of sending directly, the money travels through 4 hops."

**Point at the output:**
- Point at the 4 HOP sections
- Emphasize: "Each hop splits into 48 paths"
- Show the math building up: 48 → 2,304 → 110,592 → 5,308,416

> "At each hop, there are 48 possible paths. An observer can see all 48, but they don't know which 4 are real."

> "By hop 4, there are 5.3 million possible paths. Finding the real one is mathematically impossible."

**Timing:** ~2:30-3:00 min total

### STEP 6: Show ZEclipse Code
**Switch to VSCode, show the SDK code:**

From the demo output or create a slide showing:
```typescript
const connector = new TimingEnhancedConnector({
  rpcUrl: 'https://api.solana.com',
  maxHops: 4,              // 4 sequential hops
  maxSplitsPerHop: 4,      // 4 real splits
  fakeSplitsPerHop: 44,    // 44 fake splits
  privacyLevel: 'MAXIMUM_PRIVACY'
});

await connector.executePrivateTransfer({
  sender: aliceKeypair,
  recipient: bobPublicKey,
  amount: 1_000_000_000
});
```

**Narrate:**
> "Here's the amazing part. For developers, it's one function call. You create a connector, pass in your privacy settings, and call executePrivateTransfer."

> "That one function handles all the multi-hop routing, all the splits, all the timing randomization. Privacy becomes the default."

**Point at key lines:**
- `maxHops: 4` → "4 hops breaking the link"
- `fakeSplitsPerHop: 44` → "44 decoys per hop"
- `MAXIMUM_PRIVACY` → "Best privacy level"

**Timing:** ~2:00-2:30 min

### STEP 7: Run Demo Segment 4-5 (Comparison)

Let the demo output show these segments.

**As Segment 4 shows (the full privacy code):**
> "This is the full code. It's clean. It's simple. It's production-ready."

**When Segment 5 shows (the comparison table):**
**This is the KEY moment.** Slow down and point carefully:

**Left column (Direct Transfer):**
```
Sender Visible:      ✅ YES
Recipient Visible:   ✅ YES
Amount Visible:      ✅ YES
Privacy Score:       0/100 ❌
```

> "Direct transfer: everything is visible."

**Right column (ZEclipse):**
```
Sender Visible:      ❌ Hidden
Recipient Visible:   ❌ Hidden
Amount Visible:      ❌ Hidden
Anonymity Set:       5,308,416
Privacy Score:       94/100 ✅
```

> "ZEclipse: everything is hidden. And look at the anonymity set: 5.3 million."

> [Pause for 2 seconds]

> "That's the difference between traceable and impossible to trace."

**Timing:** ~1:30-2:00 min

### STEP 8: Run Demo Segment 6 (Animation)

Let the animation run.

**Narrate as it shows:**
> "Let's trace Alice's money through the system. We know she started here, and we know Bob received it here. But can we find the path?"

**As each hop shows:**
- Hop 1: "48 possible starting points. We have a 1 in 48 chance of guessing right."
- Hop 2: "Now the sources are mixed. 2,304 possibilities."
- Hop 3: "Timing gets randomized. We lose all correlation."
- Hop 4: "Final routing. 5.3 million total paths. We can't tell which was real."

> "Even if you trace every single transaction on the blockchain, you can't link Alice to Bob. That's the power of multi-hop routing."

**Timing:** ~1:30-2:00 min

### STEP 9: Run Demo Segment 7 (Summary)

Let the summary show.

**Narrate:**
> "So let's recap what we've shown you:"

> "Problem: Blockchain transactions are permanently traceable."

> "Solution: Multi-hop routing creates 5.3 million possible paths."

> "Code: One function call to add privacy to your app."

> "Result: Your users get military-grade privacy without sacrificing speed."

> "And here's the best part: it's production-ready. The architecture, routing, and timing obfuscation are all battle-tested. You can start using this today."

**Closing:**
> "If privacy matters to your app—whether it's payments, credentials, or any financial data—ZEclipse gives you 5.3 million anonymity set with one function call."

> "Check it out at [your repo/website]. Thanks for watching."

**Timing:** ~1:30 min

### STEP 10: Stop Recording
```bash
# In OBS Studio, click "Stop Recording"
# Wait for file to finalize (~5 seconds)
```

---

## Expected Total Runtime
- Opening: 0:30
- Segment 1-3: 6:00
- Segment 4-5: 4:30
- Segment 6-7: 3:00
- Closing: 1:00
- **Total: ~15 minutes** (perfect length)

---

## Post-Recording Checklist

- [ ] Video file created successfully
- [ ] Audio is clear (no background noise, speaking at steady volume)
- [ ] No major stumbles (if you mess up a sentence, you can edit or re-record)
- [ ] All 7 segments are present
- [ ] Terminal output is clearly visible
- [ ] Code snippets are legible

### Quick Edits (if using simple editor)
1. Trim silence at beginning/end
2. Add intro graphic/text (optional)
3. Add captions at key moments:
   - "5.3 Million Paths"
   - "Direct Transfer is Traceable"
   - "One Function Call"
4. Add background music (royalty-free, low volume)

### Upload Settings
- **Title:** "ZEclipse: Private Transactions on Solana (Privacy Demo)"
- **Description:** Include links to:
  - GitHub repo
  - Documentation
  - Technical spec
- **Tags:** solana, privacy, cryptocurrency, blockchain, demo
- **Thumbnail:** Screenshot of the 5.3M number or anonymity comparison

---

## Tips for Better Recording

### Delivery
- **Speak slowly** (people need time to read the terminal output)
- **Point at things** (don't just describe them)
- **Pause after big numbers** (5.3 million, 48^4, 94/100)
- **Use your hands** (physical pointing makes things clearer)
- **Vary your tone** (excited for solutions, concerned for problems)

### Avoid
- ❌ Reading the script word-for-word (sounds robotic)
- ❌ Speaking too fast (people can't follow along)
- ❌ Looking at the camera awkwardly (look at the demo output)
- ❌ Saying "um" and "like" too much (edit them out later)
- ❌ Long pauses where nothing is happening (keep momentum)

### Do
- ✅ Speak naturally, like you're explaining to a friend
- ✅ Point at specific numbers and code
- ✅ Pause after complex ideas (let them sink in)
- ✅ Build narrative ("Alice wants to send to Bob...")
- ✅ Show excitement about the solution

---

## If You Mess Up

**Option 1: Keep Recording**
- Just keep going, you can edit later
- Simple edits: cut out the mistake, splice clips together

**Option 2: Pause and Re-Do Segment**
- Stop OBS
- Start from beginning of that segment
- You can splice together the good take

**Option 3: Start Over**
- It's fine. Do another complete recording.
- You'll be more confident the second time.

**Most important:** Don't stress about perfection. Your genuine explanation is better than a perfect script reading.

---

## File Organization After Recording

```
/Users/jefferson/Desktop/larp-core/ZEclipse/
├── recordings/
│   ├── zeclipsesol-demo-raw.mp4        (original recording)
│   ├── zeclipsesol-demo-edited.mp4     (with basic edits)
│   └── zeclipsesol-demo-final.mp4      (ready to upload)
├── VIDEO_SCRIPT.md
├── VIDEO_TALKING_POINTS.md
├── RECORDING_SETUP.md                  (this file)
└── app/
    └── src/demos/
        └── demo-video.ts               (the demo code)
```

---

## Editing Tools (Free Options)

1. **DaVinci Resolve** (best free option)
   - Color correction
   - Simple cuts and trims
   - Easy text overlays

2. **OBS Replay Buffer** (built-in)
   - Records last 15-60 seconds
   - Auto-saves on keyboard shortcut

3. **Shotcut** (lightweight)
   - Good for basic trimming
   - Supports many formats

4. **CapCut** (modern, very easy)
   - AI-powered edits
   - Good for social media clips

---

## Final Thoughts

This demo is designed to:
1. ✅ Show a real privacy problem (blockchain traceability)
2. ✅ Show a real solution (multi-hop routing)
3. ✅ Show real code (TypeScript SDK)
4. ✅ Build from problem → solution → implementation
5. ✅ Work for both technical AND non-technical audiences

The 5.3 million anonymity set is the visual anchor that makes everything else make sense. When people understand "5.3 million possible paths," they understand why tracing is impossible.

**You've got this. The demo is ready to record.** 🚀

