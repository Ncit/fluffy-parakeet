import 'package:flutter/material.dart';

void main() {
  runApp(const FluffyApp());
}

class FluffyApp extends StatelessWidget {
  const FluffyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'Fluffy Parakeet',
      theme: ThemeData.dark(),
      home: const EditorScreen(),
    );
  }
}

class EditorScreen extends StatelessWidget {
  const EditorScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Row(
        children: const [
          Expanded(flex: 2, child: TimelinePanel()),
          Expanded(flex: 3, child: PreviewPanel()),
          Expanded(flex: 2, child: AIPanel()),
        ],
      ),
    );
  }
}

class TimelinePanel extends StatelessWidget {
  const TimelinePanel({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      color: Colors.black87,
      child: const Center(child: Text("Timeline")),
    );
  }
}

class PreviewPanel extends StatelessWidget {
  const PreviewPanel({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      color: Colors.black,
      child: const Center(child: Text("GPU Preview (WIP)")),
    );
  }
}

class AIPanel extends StatelessWidget {
  const AIPanel({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      color: Colors.black87,
      child: const Center(child: Text("AI Assistant")),
    );
  }
}
